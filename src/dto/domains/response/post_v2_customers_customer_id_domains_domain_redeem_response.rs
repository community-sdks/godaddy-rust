use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsDomainRedeemResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsDomainRedeemResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
