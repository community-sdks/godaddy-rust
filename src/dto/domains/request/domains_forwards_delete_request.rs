use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct DomainsForwardsDeleteRequest {
    pub customer_id: ParamValue,
    pub fqdn: ParamValue,
}

impl DomainsForwardsDeleteRequest {
    pub fn new(customer_id: impl Into<ParamValue>, fqdn: impl Into<ParamValue>) -> Self {
        Self {
            customer_id: customer_id.into(),
            fqdn: fqdn.into(),
        }
    }
}
