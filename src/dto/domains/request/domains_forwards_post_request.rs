use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct DomainsForwardsPostRequest {
    pub customer_id: ParamValue,
    pub fqdn: ParamValue,
    pub body: ParamValue,
}

impl DomainsForwardsPostRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        fqdn: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            fqdn: fqdn.into(),
            body: body.into(),
        }
    }
}
