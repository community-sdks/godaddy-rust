use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAgentResponse {
    pub raw: Value,
}

impl GetAgentResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
