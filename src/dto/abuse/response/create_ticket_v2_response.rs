use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CreateTicketV2Response {
    pub raw: Value,
}

impl CreateTicketV2Response {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
