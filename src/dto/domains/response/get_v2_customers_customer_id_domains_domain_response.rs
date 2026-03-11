use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsDomainResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsDomainResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
