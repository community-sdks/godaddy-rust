use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PutV2CustomersCustomerIdDomainsNotificationsOptInResponse {
    pub raw: Value,
}

impl PutV2CustomersCustomerIdDomainsNotificationsOptInResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
