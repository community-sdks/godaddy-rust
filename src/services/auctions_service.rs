use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        customer_id: impl Into<ParamValue>,
        request_body: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let request_body = request_body.into();
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
    }
}
