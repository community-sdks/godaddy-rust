use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetCustomerCertificatesByCustomerIdResponse {
    pub raw: Value,
}

impl GetCustomerCertificatesByCustomerIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
