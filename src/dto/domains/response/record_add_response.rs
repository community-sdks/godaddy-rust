use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RecordAddResponse {
    pub raw: Value,
}

impl RecordAddResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
