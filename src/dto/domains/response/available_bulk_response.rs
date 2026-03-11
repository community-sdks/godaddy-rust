use serde_json::Value;

#[derive(Clone, Debug)]
pub struct AvailableBulkResponse {
    pub raw: Value,
}

impl AvailableBulkResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
