use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct AftermarketService {
    inner: AbstractService,
}

impl AftermarketService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn get_listings(
        &self,
        request: crate::dto::aftermarket::request::GetListingsRequest,
    ) -> ApiResult<crate::dto::aftermarket::response::GetListingsResponse> {
        let customer_id = request.customer_id;
        let domains = request.domains;
        let listing_status = request.listing_status;
        let transfer_before = request.transfer_before;
        let transfer_after = request.transfer_after;
        let limit = request.limit;
        let offset = request.offset;
        self.inner
            .call(
                "GET",
                "/v1/customers/{customerId}/auctions/listings",
                vec![("customerId", Some(customer_id))],
                vec![
                    ("domains", domains),
                    ("listingStatus", listing_status),
                    ("transferBefore", transfer_before),
                    ("transferAfter", transfer_after),
                    ("limit", limit),
                    ("offset", offset),
                ],
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::aftermarket::response::GetListingsResponse::from_value)
    }

    pub async fn delete_listings(
        &self,
        request: crate::dto::aftermarket::request::DeleteListingsRequest,
    ) -> ApiResult<crate::dto::aftermarket::response::DeleteListingsResponse> {
        let domains = request.domains;
        self.inner
            .call(
                "DELETE",
                "/v1/aftermarket/listings",
                Vec::new(),
                vec![("domains", Some(domains))],
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::aftermarket::response::DeleteListingsResponse::from_value)
    }

    pub async fn add_expiry_listings(
        &self,
        request: crate::dto::aftermarket::request::AddExpiryListingsRequest,
    ) -> ApiResult<crate::dto::aftermarket::response::AddExpiryListingsResponse> {
        let expiry_listings = request.expiry_listings;
        self.inner
            .call(
                "POST",
                "/v1/aftermarket/listings/expiry",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(expiry_listings),
            )
            .await
            .map(crate::dto::aftermarket::response::AddExpiryListingsResponse::from_value)
    }
}
