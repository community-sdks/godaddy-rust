use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateRevokeResponse {
    pub raw: Value,
}

impl CertificateRevokeResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
