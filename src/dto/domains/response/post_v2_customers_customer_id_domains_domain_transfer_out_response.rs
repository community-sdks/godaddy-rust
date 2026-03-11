use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferOutResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferOutResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
