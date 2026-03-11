use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainRegenerateAuthCodeResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainRegenerateAuthCodeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
