use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RecordReplaceTypeResponse {
    pub raw: Value,
}

impl RecordReplaceTypeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
