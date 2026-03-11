use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PurchaseResponse {
    pub raw: Value,
}

impl PurchaseResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
