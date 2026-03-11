use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAgreementResponse {
    pub raw: Value,
}

impl GetAgreementResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
