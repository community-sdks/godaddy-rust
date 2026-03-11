use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
