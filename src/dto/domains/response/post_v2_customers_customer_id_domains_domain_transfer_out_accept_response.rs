use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferOutAcceptResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferOutAcceptResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
