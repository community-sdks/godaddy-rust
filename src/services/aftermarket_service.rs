use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        customer_id: impl Into<ParamValue>,
        domains: Option<ParamValue>,
        listing_status: Option<ParamValue>,
        transfer_before: Option<ParamValue>,
        transfer_after: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
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
    }

    pub async fn delete_listings(&self, domains: impl Into<ParamValue>) -> ApiResult<Value> {
        let domains = domains.into();
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
    }

    pub async fn add_expiry_listings(
        &self,
        expiry_listings: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let expiry_listings = expiry_listings.into();
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
    }
}
