use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CertificateResendEmailAddressResponse {
    pub raw: Value,
}

impl CertificateResendEmailAddressResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
