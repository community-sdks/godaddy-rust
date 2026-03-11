use serde_json::Value;

#[derive(Clone, Debug)]
pub struct VerifyEmailResponse {
    pub raw: Value,
}

impl VerifyEmailResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
