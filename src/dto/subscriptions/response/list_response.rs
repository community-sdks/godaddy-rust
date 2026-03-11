use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ListResponse {
    pub raw: Value,
}

impl ListResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
