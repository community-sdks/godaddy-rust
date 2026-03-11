use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetTicketInfoRequest {
    pub ticket_id: ParamValue,
}

impl GetTicketInfoRequest {
    pub fn new(ticket_id: impl Into<ParamValue>) -> Self {
        Self {
            ticket_id: ticket_id.into(),
        }
    }
}
