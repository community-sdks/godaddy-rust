use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DeleteV2CustomersCustomerIdDomainsDomainActionsTypeResponse {
    pub raw: Value,
}

impl DeleteV2CustomersCustomerIdDomainsDomainActionsTypeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
