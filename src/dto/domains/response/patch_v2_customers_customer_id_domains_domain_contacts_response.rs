use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PatchV2CustomersCustomerIdDomainsDomainContactsResponse {
    pub raw: Value,
}

impl PatchV2CustomersCustomerIdDomainsDomainContactsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
