use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RenewResponse {
    pub raw: Value,
}

impl RenewResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
