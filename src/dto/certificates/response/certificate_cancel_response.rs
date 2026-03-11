use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateCancelResponse {
    pub raw: Value,
}

impl CertificateCancelResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
