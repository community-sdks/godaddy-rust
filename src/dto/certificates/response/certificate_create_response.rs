use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateCreateResponse {
    pub raw: Value,
}

impl CertificateCreateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
