use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ProductGroupsRequest {
    pub x_app_key: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl ProductGroupsRequest {
    pub fn new(x_app_key: impl Into<ParamValue>, x_shopper_id: Option<ParamValue>) -> Self {
        Self {
            x_app_key: x_app_key.into(),
            x_shopper_id,
        }
    }
}
