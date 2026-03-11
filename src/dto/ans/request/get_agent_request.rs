use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetAgentRequest {
    pub agent_id: ParamValue,
}

impl GetAgentRequest {
    pub fn new(agent_id: impl Into<ParamValue>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }
}
