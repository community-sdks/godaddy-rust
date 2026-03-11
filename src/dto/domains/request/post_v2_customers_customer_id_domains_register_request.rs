use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsRegisterRequest {
    pub customer_id: ParamValue,
    pub body: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl PostV2CustomersCustomerIdDomainsRegisterRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            body: body.into(),
            x_request_id,
        }
    }
}
