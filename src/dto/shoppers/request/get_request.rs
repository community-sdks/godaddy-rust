use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetRequest {
    pub shopper_id: ParamValue,
    pub includes: Option<ParamValue>,
}

impl GetRequest {
    pub fn new(shopper_id: impl Into<ParamValue>, includes: Option<ParamValue>) -> Self {
        Self {
            shopper_id: shopper_id.into(),
            includes,
        }
    }
}
