use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CreateSubaccountResponse {
    pub raw: Value,
}

impl CreateSubaccountResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
