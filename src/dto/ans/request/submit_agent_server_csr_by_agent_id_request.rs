use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct SubmitAgentServerCsrByAgentIdRequest {
    pub agent_id: ParamValue,
    pub body: ParamValue,
}

impl SubmitAgentServerCsrByAgentIdRequest {
    pub fn new(agent_id: impl Into<ParamValue>, body: impl Into<ParamValue>) -> Self {
        Self {
            agent_id: agent_id.into(),
            body: body.into(),
        }
    }
}
