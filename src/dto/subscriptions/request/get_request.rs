use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetRequest {
    pub subscription_id: ParamValue,
    pub x_app_key: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl GetRequest {
    pub fn new(
        subscription_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            subscription_id: subscription_id.into(),
            x_app_key: x_app_key.into(),
            x_shopper_id,
        }
    }
}
