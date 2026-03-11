use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetTicketsV2Response {
    pub raw: Value,
}

impl GetTicketsV2Response {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
