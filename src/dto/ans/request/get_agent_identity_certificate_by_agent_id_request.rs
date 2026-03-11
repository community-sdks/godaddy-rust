use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetAgentIdentityCertificateByAgentIdRequest {
    pub agent_id: ParamValue,
}

impl GetAgentIdentityCertificateByAgentIdRequest {
    pub fn new(agent_id: impl Into<ParamValue>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }
}
