use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferOutRequest {
    pub customer_id: ParamValue,
    pub domain: ParamValue,
    pub registrar: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferOutRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        registrar: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            domain: domain.into(),
            registrar: registrar.into(),
            x_request_id,
        }
    }
}
