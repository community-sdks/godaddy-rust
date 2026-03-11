use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DomainsForwardsDeleteResponse {
    pub raw: Value,
}

impl DomainsForwardsDeleteResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
