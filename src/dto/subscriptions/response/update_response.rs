use serde_json::Value;

#[derive(Clone, Debug)]
pub struct UpdateResponse {
    pub raw: Value,
}

impl UpdateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
