use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RegisterAgentRequest {
    pub body: ParamValue,
}

impl RegisterAgentRequest {
    pub fn new(body: impl Into<ParamValue>) -> Self {
        Self { body: body.into() }
    }
}
