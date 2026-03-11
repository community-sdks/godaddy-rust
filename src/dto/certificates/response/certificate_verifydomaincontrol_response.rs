use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateVerifydomaincontrolResponse {
    pub raw: Value,
}

impl CertificateVerifydomaincontrolResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
