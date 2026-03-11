use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RecordReplaceResponse {
    pub raw: Value,
}

impl RecordReplaceResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
