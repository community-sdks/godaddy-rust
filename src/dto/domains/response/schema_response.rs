use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SchemaResponse {
    pub raw: Value,
}

impl SchemaResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
