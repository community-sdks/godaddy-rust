use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PlaceBidsRequest {
    pub customer_id: ParamValue,
    pub request_body: ParamValue,
}

impl PlaceBidsRequest {
    pub fn new(customer_id: impl Into<ParamValue>, request_body: impl Into<ParamValue>) -> Self {
        Self {
            customer_id: customer_id.into(),
            request_body: request_body.into(),
        }
    }
}
