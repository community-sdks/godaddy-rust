use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferInRestartResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferInRestartResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
