use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::Value;

use crate::config::Config;
use crate::error::{ApiError, ApiException, ApiResult};
use crate::http::{Request, ReqwestTransport, Response, Transport};

#[derive(Clone, Debug, PartialEq)]
pub enum ParamValue {
    String(String),
    Bool(bool),
    Number(serde_json::Number),
    Json(Value),
    Array(Vec<ParamValue>),
}

impl ParamValue {
    pub fn to_string_value(&self) -> String {
        match self {
            Self::String(value) => value.clone(),
            Self::Bool(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::Json(value) => match value {
                Value::Null => String::new(),
                Value::Bool(inner) => inner.to_string(),
                Value::Number(inner) => inner.to_string(),
                Value::String(inner) => inner.clone(),
                Value::Array(_) | Value::Object(_) => value.to_string(),
            },
            Self::Array(values) => {
                Value::Array(values.iter().map(ParamValue::to_json_value).collect()).to_string()
            }
        }
    }

    pub fn to_json_value(&self) -> Value {
        match self {
            Self::String(value) => Value::String(value.clone()),
            Self::Bool(value) => Value::Bool(*value),
            Self::Number(value) => Value::Number(value.clone()),
            Self::Json(value) => value.clone(),
            Self::Array(values) => {
                Value::Array(values.iter().map(ParamValue::to_json_value).collect())
            }
        }
    }
}

impl From<&str> for ParamValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ParamValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for ParamValue {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}

impl From<bool> for ParamValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for ParamValue {
    fn from(value: i32) -> Self {
        Self::Number(value.into())
    }
}

impl From<i64> for ParamValue {
    fn from(value: i64) -> Self {
        Self::Number(value.into())
    }
}

impl From<u32> for ParamValue {
    fn from(value: u32) -> Self {
        Self::Number(value.into())
    }
}

impl From<u64> for ParamValue {
    fn from(value: u64) -> Self {
        Self::Number(value.into())
    }
}

impl From<usize> for ParamValue {
    fn from(value: usize) -> Self {
        Self::Number((value as u64).into())
    }
}

impl From<f64> for ParamValue {
    fn from(value: f64) -> Self {
        Self::Number(serde_json::Number::from_f64(value).unwrap_or_else(|| 0.into()))
    }
}

impl From<Value> for ParamValue {
    fn from(value: Value) -> Self {
        Self::Json(value)
    }
}

impl<T> From<Vec<T>> for ParamValue
where
    T: Into<ParamValue>,
{
    fn from(values: Vec<T>) -> Self {
        Self::Array(values.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone)]
pub struct ApiClient {
    config: Config,
    transport: Arc<dyn Transport>,
}

impl ApiClient {
    pub fn new(config: Config, transport: Option<Arc<dyn Transport>>) -> Self {
        Self {
            config,
            transport: transport.unwrap_or_else(|| Arc::new(ReqwestTransport::default())),
        }
    }

    pub fn build_query_pairs(
        values: Vec<(&'static str, Option<ParamValue>)>,
    ) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        for (key, value) in values {
            let Some(value) = value else {
                continue;
            };
            match value {
                ParamValue::Array(items) => {
                    for item in items {
                        pairs.push((key.to_string(), item.to_string_value()));
                    }
                }
                other => pairs.push((key.to_string(), other.to_string_value())),
            }
        }
        pairs
    }

    pub async fn request(
        &self,
        method: &str,
        service_base_url: &str,
        path: &str,
        path_params: Vec<(&'static str, Option<ParamValue>)>,
        query_params: Vec<(&'static str, Option<ParamValue>)>,
        headers: Vec<(&'static str, Option<ParamValue>)>,
        body: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let url = format!(
            "{}{}",
            self.config
                .base_url
                .clone()
                .unwrap_or_else(|| service_base_url.to_string())
                .trim_end_matches('/'),
            self.interpolate_path(path, path_params),
        );

        let request = Request {
            method: method.to_string(),
            url,
            headers: self.build_headers(headers, body.as_ref()),
            query: Self::build_query_pairs(query_params),
            body: body.map(|value| value.to_json_value()),
            timeout: self.config.timeout,
        };

        let response = self.send_with_retry(request.clone()).await?;
        if !(200..300).contains(&response.status_code) {
            return Err(self.map_exception(&request, &response));
        }

        self.decode_response(response)
    }

    fn build_headers(
        &self,
        headers: Vec<(&'static str, Option<ParamValue>)>,
        body: Option<&ParamValue>,
    ) -> BTreeMap<String, String> {
        let mut resolved = BTreeMap::new();
        resolved.insert(String::from("Accept"), String::from("application/json"));
        resolved.insert(String::from("User-Agent"), self.config.user_agent.clone());
        resolved.extend(self.config.default_headers.clone());

        for (key, value) in headers {
            if let Some(value) = value {
                resolved.insert(key.to_string(), value.to_string_value());
            }
        }

        if let (Some(api_key), Some(api_secret)) = (&self.config.api_key, &self.config.api_secret) {
            resolved.insert(
                String::from("Authorization"),
                format!("sso-key {}:{}", api_key, api_secret),
            );
        }

        if body.is_some() && !resolved.contains_key("Content-Type") {
            resolved.insert(
                String::from("Content-Type"),
                String::from("application/json"),
            );
        }

        resolved
    }

    async fn send_with_retry(&self, request: Request) -> ApiResult<Response> {
        let mut attempt = 0;
        loop {
            let response = self.transport.send(request.clone()).await?;
            if !self.should_retry(response.status_code, attempt) {
                return Ok(response);
            }
            self.sleep_before_retry(&response, attempt).await;
            attempt += 1;
        }
    }

    fn should_retry(&self, status_code: u16, attempt: u32) -> bool {
        if attempt >= self.config.max_retries {
            return false;
        }

        matches!(status_code, 408 | 429 | 500 | 502 | 503 | 504)
    }

    async fn sleep_before_retry(&self, response: &Response, attempt: u32) {
        let base_delay = self.config.retry_delay.as_millis() as u64;
        let retry_after = response
            .headers
            .get("retry-after")
            .or_else(|| response.headers.get("Retry-After"))
            .and_then(|value| value.parse::<u64>().ok())
            .map(|value| value.saturating_mul(1000));
        let delay = retry_after.unwrap_or_else(|| base_delay.saturating_mul(2_u64.pow(attempt)));
        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
    }

    fn interpolate_path(
        &self,
        path: &str,
        path_params: Vec<(&'static str, Option<ParamValue>)>,
    ) -> String {
        let mut resolved = path.to_string();
        for (key, value) in path_params {
            let Some(value) = value else {
                continue;
            };
            let token = format!("{{{}}}", key);
            let encoded: String =
                url::form_urlencoded::byte_serialize(value.to_string_value().as_bytes()).collect();
            resolved = resolved.replace(&token, &encoded);
        }
        resolved
    }

    fn decode_response(&self, response: Response) -> ApiResult<Value> {
        if response.body.trim().is_empty() {
            return Ok(Value::Null);
        }

        let content_type = response
            .headers
            .get("content-type")
            .or_else(|| response.headers.get("Content-Type"))
            .map(|value| value.to_lowercase())
            .unwrap_or_default();
        let trimmed = response.body.trim_start();
        if content_type.contains("json") || trimmed.starts_with('{') || trimmed.starts_with('[') {
            return serde_json::from_str::<Value>(&response.body)
                .or_else(|_| Ok(Value::String(response.body.clone())))
                .map_err(|error: serde_json::Error| ApiError::Serialization(error.to_string()));
        }

        Ok(Value::String(response.body))
    }

    fn map_exception(&self, request: &Request, response: &Response) -> ApiError {
        let exception = ApiException {
            status_code: response.status_code,
            response_body: response.body.clone(),
            headers: response.headers.clone(),
            request_method: request.method.clone(),
            request_url: request.full_url(),
        };

        match response.status_code {
            400 => ApiError::Validation(exception),
            401 | 403 => ApiError::Unauthorized(exception),
            404 => ApiError::NotFound(exception),
            429 => ApiError::RateLimit(exception),
            500..=599 => ApiError::Server(exception),
            _ => ApiError::Api(exception),
        }
    }
}
