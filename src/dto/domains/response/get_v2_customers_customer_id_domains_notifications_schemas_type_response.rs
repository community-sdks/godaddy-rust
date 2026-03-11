use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
