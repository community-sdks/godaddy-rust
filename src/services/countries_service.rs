use std::sync::Arc;

use crate::api_client::ApiClient;
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

    pub async fn get_countries(
        &self,
        request: crate::dto::countries::request::GetCountriesRequest,
    ) -> ApiResult<crate::dto::countries::response::GetCountriesResponse> {
        let market_id = request.market_id;
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
            .map(crate::dto::countries::response::GetCountriesResponse::from_value)
    }

    pub async fn get_country(
        &self,
        request: crate::dto::countries::request::GetCountryRequest,
    ) -> ApiResult<crate::dto::countries::response::GetCountryResponse> {
        let country_key = request.country_key;
        let market_id = request.market_id;
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
            .map(crate::dto::countries::response::GetCountryResponse::from_value)
    }
}
