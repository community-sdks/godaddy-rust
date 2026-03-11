use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2DomainsMaintenancesMaintenanceIdResponse {
    pub raw: Value,
}

impl GetV2DomainsMaintenancesMaintenanceIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
