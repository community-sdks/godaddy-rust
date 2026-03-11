use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CreateTicketRequest {
    pub body: ParamValue,
}

impl CreateTicketRequest {
    pub fn new(body: impl Into<ParamValue>) -> Self {
        Self { body: body.into() }
    }
}
