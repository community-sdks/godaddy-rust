use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ChangePasswordResponse {
    pub raw: Value,
}

impl ChangePasswordResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
