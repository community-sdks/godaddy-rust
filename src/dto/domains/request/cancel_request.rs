use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CancelRequest {
    pub domain: ParamValue,
}

impl CancelRequest {
    pub fn new(domain: impl Into<ParamValue>) -> Self {
        Self {
            domain: domain.into(),
        }
    }
}
