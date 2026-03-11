use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PutV2CustomersCustomerIdDomainsDomainNameServersResponse {
    pub raw: Value,
}

impl PutV2CustomersCustomerIdDomainsDomainNameServersResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
