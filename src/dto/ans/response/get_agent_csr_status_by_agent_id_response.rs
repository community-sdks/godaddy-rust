use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAgentCsrStatusByAgentIdResponse {
    pub raw: Value,
}

impl GetAgentCsrStatusByAgentIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
