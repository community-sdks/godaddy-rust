use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsRegisterSchemaTldRequest {
    pub customer_id: ParamValue,
    pub tld: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl GetV2CustomersCustomerIdDomainsRegisterSchemaTldRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        tld: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            tld: tld.into(),
            x_request_id,
        }
    }
}
