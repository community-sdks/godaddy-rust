use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct CountriesService {
    inner: AbstractService,
}

impl CountriesService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn get_countries(&self, market_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let market_id = market_id.into();
        self.inner
            .call(
                "GET",
                "/v1/countries",
                Vec::new(),
                vec![("marketId", Some(market_id))],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_country(
        &self,
        country_key: impl Into<ParamValue>,
        market_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let country_key = country_key.into();
        let market_id = market_id.into();
        self.inner
            .call(
                "GET",
                "/v1/countries/{countryKey}",
                vec![("countryKey", Some(country_key))],
                vec![("marketId", Some(market_id))],
                Vec::new(),
                None,
            )
            .await
    }
}
