use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetCountriesResponse {
    pub raw: Value,
}

impl GetCountriesResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
