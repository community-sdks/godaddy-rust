use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ProductGroupsResponse {
    pub raw: Value,
}

impl ProductGroupsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
