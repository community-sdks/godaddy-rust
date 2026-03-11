use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CancelResponse {
    pub raw: Value,
}

impl CancelResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
