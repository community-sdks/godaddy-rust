use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetRequest {
    pub keys: ParamValue,
    pub x_private_label_id: Option<ParamValue>,
    pub x_market_id: Option<ParamValue>,
}

impl GetRequest {
    pub fn new(
        keys: impl Into<ParamValue>,
        x_private_label_id: Option<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> Self {
        Self {
            keys: keys.into(),
            x_private_label_id,
            x_market_id,
        }
    }
}
