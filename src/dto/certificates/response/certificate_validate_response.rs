use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateValidateResponse {
    pub raw: Value,
}

impl CertificateValidateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
