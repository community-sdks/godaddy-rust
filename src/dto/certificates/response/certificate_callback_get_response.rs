use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateCallbackGetResponse {
    pub raw: Value,
}

impl CertificateCallbackGetResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
