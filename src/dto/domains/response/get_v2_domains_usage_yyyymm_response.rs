use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2DomainsUsageYyyymmResponse {
    pub raw: Value,
}

impl GetV2DomainsUsageYyyymmResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
