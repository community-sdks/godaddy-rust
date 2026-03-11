use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ContactsValidateRequest {
    pub body: ParamValue,
    pub x_private_label_id: Option<ParamValue>,
    pub market_id: Option<ParamValue>,
}

impl ContactsValidateRequest {
    pub fn new(
        body: impl Into<ParamValue>,
        x_private_label_id: Option<ParamValue>,
        market_id: Option<ParamValue>,
    ) -> Self {
        Self {
            body: body.into(),
            x_private_label_id,
            market_id,
        }
    }
}
