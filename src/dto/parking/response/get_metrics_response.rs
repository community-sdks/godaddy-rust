use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetMetricsResponse {
    pub raw: Value,
}

impl GetMetricsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
