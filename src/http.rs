use std::collections::BTreeMap;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::error::{ApiError, ApiResult};

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub query: Vec<(String, String)>,
    pub body: Option<serde_json::Value>,
    pub timeout: Duration,
}

impl Request {
    pub fn full_url(&self) -> String {
        if self.query.is_empty() {
            return self.url.clone();
        }

        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for (key, value) in &self.query {
            serializer.append_pair(key, value);
        }
        format!("{}?{}", self.url, serializer.finish())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub status_code: u16,
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn json(status_code: u16, body: &str) -> Self {
        let mut headers = BTreeMap::new();
        headers.insert(
            String::from("content-type"),
            String::from("application/json"),
        );
        Self {
            status_code,
            headers,
            body: body.to_string(),
        }
    }
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, request: Request) -> ApiResult<Response>;
}

#[derive(Debug)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl Default for ReqwestTransport {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Transport for ReqwestTransport {
    async fn send(&self, request: Request) -> ApiResult<Response> {
        let mut headers = HeaderMap::new();
        for (key, value) in &request.headers {
            let header_name = HeaderName::try_from(key.as_str())
                .map_err(|error| ApiError::Transport(error.to_string()))?;
            let header_value = HeaderValue::try_from(value.as_str())
                .map_err(|error| ApiError::Transport(error.to_string()))?;
            headers.insert(header_name, header_value);
        }

        let mut builder = self
            .client
            .request(
                reqwest::Method::from_bytes(request.method.as_bytes())
                    .map_err(|error| ApiError::Transport(error.to_string()))?,
                request.url.as_str(),
            )
            .headers(headers)
            .timeout(request.timeout)
            .query(&request.query);

        if let Some(body) = &request.body {
            builder = builder.json(body);
        }

        let response = builder
            .send()
            .await
            .map_err(|error| ApiError::Transport(error.to_string()))?;

        let status_code = response.status().as_u16();
        let mut response_headers = BTreeMap::new();
        for (key, value) in response.headers() {
            response_headers.insert(
                key.as_str().to_string(),
                value.to_str().unwrap_or_default().to_string(),
            );
        }

        let body = response
            .text()
            .await
            .map_err(|error| ApiError::Transport(error.to_string()))?;

        Ok(Response {
            status_code,
            headers: response_headers,
            body,
        })
    }
}
