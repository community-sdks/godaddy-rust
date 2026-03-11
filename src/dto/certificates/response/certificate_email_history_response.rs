use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateEmailHistoryResponse {
    pub raw: Value,
}

impl CertificateEmailHistoryResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
