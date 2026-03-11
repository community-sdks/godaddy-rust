use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetDomainInformationByCertificateIdResponse {
    pub raw: Value,
}

impl GetDomainInformationByCertificateIdResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
