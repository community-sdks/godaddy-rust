use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ChangePasswordRequest {
    pub shopper_id: ParamValue,
    pub secret: ParamValue,
}

impl ChangePasswordRequest {
    pub fn new(shopper_id: impl Into<ParamValue>, secret: impl Into<ParamValue>) -> Self {
        Self {
            shopper_id: shopper_id.into(),
            secret: secret.into(),
        }
    }
}
