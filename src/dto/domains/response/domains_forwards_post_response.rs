use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DomainsForwardsPostResponse {
    pub raw: Value,
}

impl DomainsForwardsPostResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
