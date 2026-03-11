use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct AuctionsService {
    inner: AbstractService,
}

impl AuctionsService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn place_bids(
        &self,
        request: crate::dto::auctions::request::PlaceBidsRequest,
    ) -> ApiResult<crate::dto::auctions::response::PlaceBidsResponse> {
        let customer_id = request.customer_id;
        let request_body = request.request_body;
        self.inner
            .call(
                "POST",
                "/v1/customers/{customerId}/aftermarket/listings/bids",
                vec![("customerId", Some(customer_id))],
                Vec::new(),
                Vec::new(),
                Some(request_body),
            )
            .await
            .map(crate::dto::auctions::response::PlaceBidsResponse::from_value)
    }
}
