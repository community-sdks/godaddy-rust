use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsDomainActionsTypeResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsDomainActionsTypeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
