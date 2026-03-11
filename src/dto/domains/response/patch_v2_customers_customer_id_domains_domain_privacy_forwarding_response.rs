use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PatchV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse {
    pub raw: Value,
}

impl PatchV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
