use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsNotificationsResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsNotificationsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
