use serde_json::Value;

#[derive(Clone, Debug)]
pub struct AddExpiryListingsResponse {
    pub raw: Value,
}

impl AddExpiryListingsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
