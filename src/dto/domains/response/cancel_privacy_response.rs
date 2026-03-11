use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CancelPrivacyResponse {
    pub raw: Value,
}

impl CancelPrivacyResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
