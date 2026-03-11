use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainRenewRequest {
    pub customer_id: ParamValue,
    pub domain: ParamValue,
    pub body: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl PostV2CustomersCustomerIdDomainsDomainRenewRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            domain: domain.into(),
            body: body.into(),
            x_request_id,
        }
    }
}
