use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainTransferValidateResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainTransferValidateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
