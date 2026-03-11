use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct DomainsForwardsGetRequest {
    pub customer_id: ParamValue,
    pub fqdn: ParamValue,
    pub include_subs: Option<ParamValue>,
}

impl DomainsForwardsGetRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        fqdn: impl Into<ParamValue>,
        include_subs: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            fqdn: fqdn.into(),
            include_subs,
        }
    }
}
