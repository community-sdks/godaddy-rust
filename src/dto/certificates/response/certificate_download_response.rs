use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateDownloadResponse {
    pub raw: Value,
}

impl CertificateDownloadResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
