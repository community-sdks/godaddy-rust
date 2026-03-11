use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetTicketInfoV2Response {
    pub raw: Value,
}

impl GetTicketInfoV2Response {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
