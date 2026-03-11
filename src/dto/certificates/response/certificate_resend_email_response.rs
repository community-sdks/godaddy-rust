use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateResendEmailResponse {
    pub raw: Value,
}

impl CertificateResendEmailResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
