use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferInAcceptResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferInAcceptResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
