use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ContactsValidateResponse {
    pub raw: Value,
}

impl ContactsValidateResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
