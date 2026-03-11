use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateDownloadEntitlementResponse {
    pub raw: Value,
}

impl CertificateDownloadEntitlementResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
