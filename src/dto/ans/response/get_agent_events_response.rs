use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAgentEventsResponse {
    pub raw: Value,
}

impl GetAgentEventsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
