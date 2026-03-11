use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetAgentCsrStatusByAgentIdRequest {
    pub agent_id: ParamValue,
    pub csr_id: ParamValue,
}

impl GetAgentCsrStatusByAgentIdRequest {
    pub fn new(agent_id: impl Into<ParamValue>, csr_id: impl Into<ParamValue>) -> Self {
        Self {
            agent_id: agent_id.into(),
            csr_id: csr_id.into(),
        }
    }
}
