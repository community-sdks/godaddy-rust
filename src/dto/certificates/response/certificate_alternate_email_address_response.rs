use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateAlternateEmailAddressResponse {
    pub raw: Value,
}

impl CertificateAlternateEmailAddressResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
