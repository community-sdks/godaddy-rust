use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DeleteListingsResponse {
    pub raw: Value,
}

impl DeleteListingsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
