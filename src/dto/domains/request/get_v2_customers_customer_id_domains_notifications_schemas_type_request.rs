use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeRequest {
    pub customer_id: ParamValue,
    pub type_: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            type_: type_.into(),
            x_request_id,
        }
    }
}
