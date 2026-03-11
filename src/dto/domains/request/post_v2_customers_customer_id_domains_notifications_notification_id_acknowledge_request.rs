use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeRequest {
    pub customer_id: ParamValue,
    pub notification_id: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        notification_id: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            notification_id: notification_id.into(),
            x_request_id,
        }
    }
}
