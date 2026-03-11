use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetMetricsByDomainResponse {
    pub raw: Value,
}

impl GetMetricsByDomainResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
