use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetResponse {
    pub raw: Value,
}

impl GetResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
