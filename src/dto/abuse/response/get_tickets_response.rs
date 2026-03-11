use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetTicketsResponse {
    pub raw: Value,
}

impl GetTicketsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
