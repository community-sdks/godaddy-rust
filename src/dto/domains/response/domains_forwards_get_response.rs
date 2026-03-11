use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DomainsForwardsGetResponse {
    pub raw: Value,
}

impl DomainsForwardsGetResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
