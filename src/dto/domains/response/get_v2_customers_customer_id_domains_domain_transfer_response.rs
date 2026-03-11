use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsDomainTransferResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsDomainTransferResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
