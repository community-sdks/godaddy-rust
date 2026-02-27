use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct ParkingService {
    inner: AbstractService,
}

impl ParkingService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn get_metrics(
        &self,
        customer_id: impl Into<ParamValue>,
        period_start_ptz: Option<ParamValue>,
        period_end_ptz: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        self.inner
            .call(
                "GET",
                "/v1/customers/{customerId}/parking/metrics",
                vec![("customerId", Some(customer_id))],
                vec![
                    ("periodStartPtz", period_start_ptz),
                    ("periodEndPtz", period_end_ptz),
                    ("limit", limit),
                    ("offset", offset),
                ],
                vec![("X-Request-Id", x_request_id)],
                None,
            )
            .await
    }

    pub async fn get_metrics_by_domain(
        &self,
        customer_id: impl Into<ParamValue>,
        start_date: impl Into<ParamValue>,
        end_date: impl Into<ParamValue>,
        domains: Option<ParamValue>,
        domain_like: Option<ParamValue>,
        portfolio_id: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let start_date = start_date.into();
        let end_date = end_date.into();
        self.inner
            .call(
                "GET",
                "/v1/customers/{customerId}/parking/metricsByDomain",
                vec![("customerId", Some(customer_id))],
                vec![
                    ("startDate", Some(start_date)),
                    ("endDate", Some(end_date)),
                    ("domains", domains),
                    ("domainLike", domain_like),
                    ("portfolioId", portfolio_id),
                    ("limit", limit),
                    ("offset", offset),
                ],
                vec![("X-Request-Id", x_request_id)],
                None,
            )
            .await
    }
}
