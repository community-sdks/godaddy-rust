use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsDomainActionsTypeRequest {
    pub customer_id: ParamValue,
    pub domain: ParamValue,
    pub type_: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl GetV2CustomersCustomerIdDomainsDomainActionsTypeRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            domain: domain.into(),
            type_: type_.into(),
            x_request_id,
        }
    }
}
