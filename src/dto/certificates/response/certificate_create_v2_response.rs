use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateCreateV2Response {
    pub raw: Value,
}

impl CertificateCreateV2Response {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
