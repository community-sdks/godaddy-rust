use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateSitesealGetResponse {
    pub raw: Value,
}

impl CertificateSitesealGetResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
