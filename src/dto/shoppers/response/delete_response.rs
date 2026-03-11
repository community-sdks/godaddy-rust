use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DeleteResponse {
    pub raw: Value,
}

impl DeleteResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
