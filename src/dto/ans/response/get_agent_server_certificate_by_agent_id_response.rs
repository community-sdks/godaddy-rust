use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAgentServerCertificateByAgentIdResponse {
    pub raw: Value,
}

impl GetAgentServerCertificateByAgentIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
