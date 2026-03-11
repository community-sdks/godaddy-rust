use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
