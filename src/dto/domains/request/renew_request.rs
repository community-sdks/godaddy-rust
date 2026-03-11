use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RenewRequest {
    pub domain: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
    pub body: Option<ParamValue>,
}

impl RenewRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        body: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            x_shopper_id,
            body,
        }
    }
}
