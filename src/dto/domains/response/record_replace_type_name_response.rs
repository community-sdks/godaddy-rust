use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RecordReplaceTypeNameResponse {
    pub raw: Value,
}

impl RecordReplaceTypeNameResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
