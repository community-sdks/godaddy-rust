use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetCertificateDetailByCertIdentifierResponse {
    pub raw: Value,
}

impl GetCertificateDetailByCertIdentifierResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
