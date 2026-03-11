use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetStatusResponse {
    pub raw: Value,
}

impl GetStatusResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
