use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateGetResponse {
    pub raw: Value,
}

impl CertificateGetResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
