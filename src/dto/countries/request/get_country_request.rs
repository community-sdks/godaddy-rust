use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetCountryRequest {
    pub country_key: ParamValue,
    pub market_id: ParamValue,
}

impl GetCountryRequest {
    pub fn new(country_key: impl Into<ParamValue>, market_id: impl Into<ParamValue>) -> Self {
        Self {
            country_key: country_key.into(),
            market_id: market_id.into(),
        }
    }
}
