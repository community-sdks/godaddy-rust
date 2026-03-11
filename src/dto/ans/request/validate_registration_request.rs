use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ValidateRegistrationRequest {
    pub agent_id: ParamValue,
}

impl ValidateRegistrationRequest {
    pub fn new(agent_id: impl Into<ParamValue>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }
}
