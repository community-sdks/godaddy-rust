use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetRequest {
    pub domain: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl GetRequest {
    pub fn new(domain: impl Into<ParamValue>, x_shopper_id: Option<ParamValue>) -> Self {
        Self {
            domain: domain.into(),
            x_shopper_id,
        }
    }
}
