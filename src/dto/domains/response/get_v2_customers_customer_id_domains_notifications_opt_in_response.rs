use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsNotificationsOptInResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsNotificationsOptInResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
