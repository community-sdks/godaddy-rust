use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct DeleteListingsRequest {
    pub domains: ParamValue,
}

impl DeleteListingsRequest {
    pub fn new(domains: impl Into<ParamValue>) -> Self {
        Self {
            domains: domains.into(),
        }
    }
}
