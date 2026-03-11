use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SubmitAgentIdentityCsrByAgentIdResponse {
    pub raw: Value,
}

impl SubmitAgentIdentityCsrByAgentIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
