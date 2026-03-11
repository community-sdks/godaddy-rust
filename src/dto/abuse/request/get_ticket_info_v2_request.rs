use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetTicketInfoV2Request {
    pub ticket_id: ParamValue,
}

impl GetTicketInfoV2Request {
    pub fn new(ticket_id: impl Into<ParamValue>) -> Self {
        Self {
            ticket_id: ticket_id.into(),
        }
    }
}
