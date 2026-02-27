use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiException {
    pub status_code: u16,
    pub response_body: String,
    pub headers: BTreeMap<String, String>,
    pub request_method: String,
    pub request_url: String,
}

impl ApiException {
    pub fn request_id(&self) -> Option<&str> {
        self.headers
            .get("x-request-id")
            .map(String::as_str)
            .or_else(|| self.headers.get("X-Request-Id").map(String::as_str))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    Validation(ApiException),
    Unauthorized(ApiException),
    NotFound(ApiException),
    RateLimit(ApiException),
    Server(ApiException),
    Api(ApiException),
    Transport(String),
    Serialization(String),
}

impl ApiError {
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Validation(inner)
            | Self::Unauthorized(inner)
            | Self::NotFound(inner)
            | Self::RateLimit(inner)
            | Self::Server(inner)
            | Self::Api(inner) => Some(inner.status_code),
            Self::Transport(_) | Self::Serialization(_) => None,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(inner)
            | Self::Unauthorized(inner)
            | Self::NotFound(inner)
            | Self::RateLimit(inner)
            | Self::Server(inner)
            | Self::Api(inner) => write!(
                f,
                "GoDaddy API request failed with status {}",
                inner.status_code
            ),
            Self::Transport(message) => write!(f, "Transport error: {}", message),
            Self::Serialization(message) => write!(f, "Serialization error: {}", message),
        }
    }
}

impl Error for ApiError {}
