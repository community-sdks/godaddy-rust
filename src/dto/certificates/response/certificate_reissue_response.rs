use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateReissueResponse {
    pub raw: Value,
}

impl CertificateReissueResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
