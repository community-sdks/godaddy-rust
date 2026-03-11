use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2DomainsMaintenancesResponse {
    pub raw: Value,
}

impl GetV2DomainsMaintenancesResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
