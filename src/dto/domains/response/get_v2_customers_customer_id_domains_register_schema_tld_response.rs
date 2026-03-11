use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetV2CustomersCustomerIdDomainsRegisterSchemaTldResponse {
    pub raw: Value,
}

impl GetV2CustomersCustomerIdDomainsRegisterSchemaTldResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
