use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SubmitAgentServerCsrByAgentIdResponse {
    pub raw: Value,
}

impl SubmitAgentServerCsrByAgentIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
