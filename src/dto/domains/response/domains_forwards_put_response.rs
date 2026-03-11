use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DomainsForwardsPutResponse {
    pub raw: Value,
}

impl DomainsForwardsPutResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
