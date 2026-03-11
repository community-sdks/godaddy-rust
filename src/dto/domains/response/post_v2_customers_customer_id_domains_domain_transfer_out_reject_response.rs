use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferOutRejectResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferOutRejectResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
