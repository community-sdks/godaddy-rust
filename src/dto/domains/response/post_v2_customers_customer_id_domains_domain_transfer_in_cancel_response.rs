use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferInCancelResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferInCancelResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
