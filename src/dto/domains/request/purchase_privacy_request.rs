use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PurchasePrivacyRequest {
    pub domain: ParamValue,
    pub body: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl PurchasePrivacyRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            body: body.into(),
            x_shopper_id,
        }
    }
}
