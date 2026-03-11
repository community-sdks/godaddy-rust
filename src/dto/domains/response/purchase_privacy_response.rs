use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PurchasePrivacyResponse {
    pub raw: Value,
}

impl PurchasePrivacyResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
