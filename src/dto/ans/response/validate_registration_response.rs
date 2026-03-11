use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ValidateRegistrationResponse {
    pub raw: Value,
}

impl ValidateRegistrationResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
