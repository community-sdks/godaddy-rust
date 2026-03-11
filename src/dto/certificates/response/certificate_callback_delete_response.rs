use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateCallbackDeleteResponse {
    pub raw: Value,
}

impl CertificateCallbackDeleteResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
