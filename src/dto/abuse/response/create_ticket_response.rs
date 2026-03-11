use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CreateTicketResponse {
    pub raw: Value,
}

impl CreateTicketResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
