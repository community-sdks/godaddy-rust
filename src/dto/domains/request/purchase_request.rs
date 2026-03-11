use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PurchaseRequest {
    pub body: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl PurchaseRequest {
    pub fn new(body: impl Into<ParamValue>, x_shopper_id: Option<ParamValue>) -> Self {
        Self {
            body: body.into(),
            x_shopper_id,
        }
    }
}
