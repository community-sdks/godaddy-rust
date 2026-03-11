use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateActionRetrieveResponse {
    pub raw: Value,
}

impl CertificateActionRetrieveResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
