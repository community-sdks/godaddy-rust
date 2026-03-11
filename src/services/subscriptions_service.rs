use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct SubscriptionsService {
    inner: AbstractService,
}

impl SubscriptionsService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn list(
        &self,
        request: crate::dto::subscriptions::request::ListRequest,
    ) -> ApiResult<crate::dto::subscriptions::response::ListResponse> {
        let x_app_key = request.x_app_key;
        let x_shopper_id = request.x_shopper_id;
        let x_market_id = request.x_market_id;
        let product_group_keys = request.product_group_keys;
        let includes = request.includes;
        let offset = request.offset;
        let limit = request.limit;
        let sort = request.sort;
        self.inner
            .call(
                "GET",
                "/v1/subscriptions",
                Vec::new(),
                vec![
                    ("productGroupKeys", product_group_keys),
                    ("includes", includes),
                    ("offset", offset),
                    ("limit", limit),
                    ("sort", sort),
                ],
                vec![
                    ("X-App-Key", Some(x_app_key)),
                    ("X-Shopper-Id", x_shopper_id),
                    ("X-Market-Id", x_market_id),
                ],
                None,
            )
            .await
            .map(crate::dto::subscriptions::response::ListResponse::from_value)
    }

    pub async fn product_groups(
        &self,
        request: crate::dto::subscriptions::request::ProductGroupsRequest,
    ) -> ApiResult<crate::dto::subscriptions::response::ProductGroupsResponse> {
        let x_app_key = request.x_app_key;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "GET",
                "/v1/subscriptions/productGroups",
                Vec::new(),
                Vec::new(),
                vec![
                    ("X-App-Key", Some(x_app_key)),
                    ("X-Shopper-Id", x_shopper_id),
                ],
                None,
            )
            .await
            .map(crate::dto::subscriptions::response::ProductGroupsResponse::from_value)
    }

    pub async fn cancel(
        &self,
        request: crate::dto::subscriptions::request::CancelRequest,
    ) -> ApiResult<crate::dto::subscriptions::response::CancelResponse> {
        let subscription_id = request.subscription_id;
        let x_app_key = request.x_app_key;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "DELETE",
                "/v1/subscriptions/{subscriptionId}",
                vec![("subscriptionId", Some(subscription_id))],
                Vec::new(),
                vec![
                    ("X-App-Key", Some(x_app_key)),
                    ("X-Shopper-Id", x_shopper_id),
                ],
                None,
            )
            .await
            .map(crate::dto::subscriptions::response::CancelResponse::from_value)
    }

    pub async fn get(
        &self,
        request: crate::dto::subscriptions::request::GetRequest,
    ) -> ApiResult<crate::dto::subscriptions::response::GetResponse> {
        let subscription_id = request.subscription_id;
        let x_app_key = request.x_app_key;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "GET",
                "/v1/subscriptions/{subscriptionId}",
                vec![("subscriptionId", Some(subscription_id))],
                Vec::new(),
                vec![
                    ("X-App-Key", Some(x_app_key)),
                    ("X-Shopper-Id", x_shopper_id),
                ],
                None,
            )
            .await
            .map(crate::dto::subscriptions::response::GetResponse::from_value)
    }

    pub async fn update(
        &self,
        request: crate::dto::subscriptions::request::UpdateRequest,
    ) -> ApiResult<crate::dto::subscriptions::response::UpdateResponse> {
        let subscription_id = request.subscription_id;
        let x_app_key = request.x_app_key;
        let subscription = request.subscription;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PATCH",
                "/v1/subscriptions/{subscriptionId}",
                vec![("subscriptionId", Some(subscription_id))],
                Vec::new(),
                vec![
                    ("X-App-Key", Some(x_app_key)),
                    ("X-Shopper-Id", x_shopper_id),
                ],
                Some(subscription),
            )
            .await
            .map(crate::dto::subscriptions::response::UpdateResponse::from_value)
    }
}
