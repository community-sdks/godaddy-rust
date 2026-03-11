use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferOutRejectRequest {
    pub customer_id: ParamValue,
    pub domain: ParamValue,
    pub x_request_id: Option<ParamValue>,
    pub reason: Option<ParamValue>,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferOutRejectRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
        reason: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            domain: domain.into(),
            x_request_id,
            reason,
        }
    }
}
