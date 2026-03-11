use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RecordGetResponse {
    pub raw: Value,
}

impl RecordGetResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
