use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferInRetryResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferInRetryResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
