use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RegisterAgentResponse {
    pub raw: Value,
}

impl RegisterAgentResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
