use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CreateTicketV2Request {
    pub body: ParamValue,
}

impl CreateTicketV2Request {
    pub fn new(body: impl Into<ParamValue>) -> Self {
        Self { body: body.into() }
    }
}
