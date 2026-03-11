use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetRequest {
    pub order_id: ParamValue,
    pub x_app_key: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
    pub x_market_id: Option<ParamValue>,
}

impl GetRequest {
    pub fn new(
        order_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            x_app_key: x_app_key.into(),
            x_shopper_id,
            x_market_id,
        }
    }
}
