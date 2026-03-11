use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
