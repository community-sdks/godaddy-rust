use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ValidateRequest {
    pub body: ParamValue,
}

impl ValidateRequest {
    pub fn new(body: impl Into<ParamValue>) -> Self {
        Self { body: body.into() }
    }
}
