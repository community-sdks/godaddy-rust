use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct VerifyEmailRequest {
    pub domain: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl VerifyEmailRequest {
    pub fn new(domain: impl Into<ParamValue>, x_shopper_id: Option<ParamValue>) -> Self {
        Self {
            domain: domain.into(),
            x_shopper_id,
        }
    }
}
