use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PlaceBidsResponse {
    pub raw: Value,
}

impl PlaceBidsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
