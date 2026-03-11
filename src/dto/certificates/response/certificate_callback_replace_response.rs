use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateCallbackReplaceResponse {
    pub raw: Value,
}

impl CertificateCallbackReplaceResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
