use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PutV2CustomersCustomerIdDomainsNotificationsOptInRequest {
    pub customer_id: ParamValue,
    pub types: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl PutV2CustomersCustomerIdDomainsNotificationsOptInRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        types: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            types: types.into(),
            x_request_id,
        }
    }
}
