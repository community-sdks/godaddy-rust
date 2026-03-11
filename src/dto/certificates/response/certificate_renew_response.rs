use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateRenewResponse {
    pub raw: Value,
}

impl CertificateRenewResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
