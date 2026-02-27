use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        x_app_key: impl Into<ParamValue>,
        period_start: Option<ParamValue>,
        period_end: Option<ParamValue>,
        domain: Option<ParamValue>,
        product_group_id: Option<ParamValue>,
        payment_profile_id: Option<ParamValue>,
        parent_order_id: Option<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
        sort: Option<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let x_app_key = x_app_key.into();
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
    }

    pub async fn get(
        &self,
        order_id: impl Into<ParamValue>,
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let order_id = order_id.into();
        let x_app_key = x_app_key.into();
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
    }
}
