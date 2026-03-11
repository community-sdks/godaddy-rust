use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct UpdateRequest {
    pub shopper_id: ParamValue,
    pub shopper: ParamValue,
}

impl UpdateRequest {
    pub fn new(shopper_id: impl Into<ParamValue>, shopper: impl Into<ParamValue>) -> Self {
        Self {
            shopper_id: shopper_id.into(),
            shopper: shopper.into(),
        }
    }
}
