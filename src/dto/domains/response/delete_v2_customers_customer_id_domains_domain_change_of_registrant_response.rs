use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DeleteV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse {
    pub raw: Value,
}

impl DeleteV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
