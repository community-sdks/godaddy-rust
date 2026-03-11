use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RecordDeleteTypeNameResponse {
    pub raw: Value,
}

impl RecordDeleteTypeNameResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
