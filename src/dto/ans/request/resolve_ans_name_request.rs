use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ResolveAnsNameRequest {
    pub body: ParamValue,
}

impl ResolveAnsNameRequest {
    pub fn new(body: impl Into<ParamValue>) -> Self {
        Self { body: body.into() }
    }
}
