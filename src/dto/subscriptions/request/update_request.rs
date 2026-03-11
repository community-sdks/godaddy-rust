use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct UpdateRequest {
    pub subscription_id: ParamValue,
    pub x_app_key: ParamValue,
    pub subscription: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl UpdateRequest {
    pub fn new(
        subscription_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        subscription: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            subscription_id: subscription_id.into(),
            x_app_key: x_app_key.into(),
            subscription: subscription.into(),
            x_shopper_id,
        }
    }
}
