use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SuggestResponse {
    pub raw: Value,
}

impl SuggestResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
