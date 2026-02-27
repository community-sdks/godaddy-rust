use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        x_market_id: Option<ParamValue>,
        product_group_keys: Option<ParamValue>,
        includes: Option<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
        sort: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let x_app_key = x_app_key.into();
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
    }

    pub async fn product_groups(
        &self,
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let x_app_key = x_app_key.into();
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
    }

    pub async fn cancel(
        &self,
        subscription_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let subscription_id = subscription_id.into();
        let x_app_key = x_app_key.into();
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
    }

    pub async fn get(
        &self,
        subscription_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let subscription_id = subscription_id.into();
        let x_app_key = x_app_key.into();
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
    }

    pub async fn update(
        &self,
        subscription_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        subscription: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let subscription_id = subscription_id.into();
        let x_app_key = x_app_key.into();
        let subscription = subscription.into();
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
    }
}
