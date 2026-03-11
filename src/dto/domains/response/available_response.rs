use serde_json::Value;

#[derive(Clone, Debug)]
pub struct AvailableResponse {
    pub raw: Value,
}

impl AvailableResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
