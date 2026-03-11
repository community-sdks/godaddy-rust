use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetAcmeExternalAccountBindingRequest {
    pub customer_id: ParamValue,
}

impl GetAcmeExternalAccountBindingRequest {
    pub fn new(customer_id: impl Into<ParamValue>) -> Self {
        Self {
            customer_id: customer_id.into(),
        }
    }
}
