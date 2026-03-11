use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct SchemaRequest {
    pub tld: ParamValue,
}

impl SchemaRequest {
    pub fn new(tld: impl Into<ParamValue>) -> Self {
        Self { tld: tld.into() }
    }
}
