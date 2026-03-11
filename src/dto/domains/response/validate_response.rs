use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ValidateResponse {
    pub raw: Value,
}

impl ValidateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
