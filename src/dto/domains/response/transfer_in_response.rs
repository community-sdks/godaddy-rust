use serde_json::Value;

#[derive(Clone, Debug)]
pub struct TransferInResponse {
    pub raw: Value,
}

impl TransferInResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
