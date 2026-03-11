use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct UpdateContactsRequest {
    pub domain: ParamValue,
    pub contacts: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl UpdateContactsRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        contacts: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            contacts: contacts.into(),
            x_shopper_id,
        }
    }
}
