use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct OrdersService {
    inner: AbstractService,
}

impl OrdersService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn list(
        &self,
        request: crate::dto::orders::request::ListRequest,
    ) -> ApiResult<crate::dto::orders::response::ListResponse> {
        let x_app_key = request.x_app_key;
        let period_start = request.period_start;
        let period_end = request.period_end;
        let domain = request.domain;
        let product_group_id = request.product_group_id;
        let payment_profile_id = request.payment_profile_id;
        let parent_order_id = request.parent_order_id;
        let offset = request.offset;
        let limit = request.limit;
        let sort = request.sort;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "GET",
                "/v1/orders",
                Vec::new(),
                vec![
                    ("periodStart", period_start),
                    ("periodEnd", period_end),
                    ("domain", domain),
                    ("productGroupId", product_group_id),
                    ("paymentProfileId", payment_profile_id),
                    ("parentOrderId", parent_order_id),
                    ("offset", offset),
                    ("limit", limit),
                    ("sort", sort),
                ],
                vec![
                    ("X-Shopper-Id", x_shopper_id),
                    ("X-App-Key", Some(x_app_key)),
                ],
                None,
            )
            .await
            .map(crate::dto::orders::response::ListResponse::from_value)
    }

    pub async fn get(
        &self,
        request: crate::dto::orders::request::GetRequest,
    ) -> ApiResult<crate::dto::orders::response::GetResponse> {
        let order_id = request.order_id;
        let x_app_key = request.x_app_key;
        let x_shopper_id = request.x_shopper_id;
        let x_market_id = request.x_market_id;
        self.inner
            .call(
                "GET",
                "/v1/orders/{orderId}",
                vec![("orderId", Some(order_id))],
                Vec::new(),
                vec![
                    ("X-Shopper-Id", x_shopper_id),
                    ("X-Market-Id", x_market_id),
                    ("X-App-Key", Some(x_app_key)),
                ],
                None,
            )
            .await
            .map(crate::dto::orders::response::GetResponse::from_value)
    }
}
