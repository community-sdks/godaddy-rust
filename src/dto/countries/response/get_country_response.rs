use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetCountryResponse {
    pub raw: Value,
}

impl GetCountryResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
