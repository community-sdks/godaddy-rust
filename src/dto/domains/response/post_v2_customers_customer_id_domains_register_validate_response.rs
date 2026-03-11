use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PostV2CustomersCustomerIdDomainsRegisterValidateResponse {
    pub raw: Value,
}

impl PostV2CustomersCustomerIdDomainsRegisterValidateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
