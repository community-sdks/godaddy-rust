use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateGetEntitlementResponse {
    pub raw: Value,
}

impl CertificateGetEntitlementResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
