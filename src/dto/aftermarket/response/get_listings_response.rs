use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetListingsResponse {
    pub raw: Value,
}

impl GetListingsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
