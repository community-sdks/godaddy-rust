use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAgentIdentityCertificateByAgentIdResponse {
    pub raw: Value,
}

impl GetAgentIdentityCertificateByAgentIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
