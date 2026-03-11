use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetAcmeExternalAccountBindingResponse {
    pub raw: Value,
}

impl GetAcmeExternalAccountBindingResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
