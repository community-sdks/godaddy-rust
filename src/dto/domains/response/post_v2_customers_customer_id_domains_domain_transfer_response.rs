use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
