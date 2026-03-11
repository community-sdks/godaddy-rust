use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainRenewResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainRenewResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
