use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetDomainDetailsByDomainRequest {
    pub customer_id: ParamValue,
    pub certificate_id: ParamValue,
    pub domain: ParamValue,
}

impl GetDomainDetailsByDomainRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        certificate_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            certificate_id: certificate_id.into(),
            domain: domain.into(),
        }
    }
}
