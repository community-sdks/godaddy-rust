use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetCountriesRequest {
    pub market_id: ParamValue,
}

impl GetCountriesRequest {
    pub fn new(market_id: impl Into<ParamValue>) -> Self {
        Self {
            market_id: market_id.into(),
        }
    }
}
