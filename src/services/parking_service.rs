use std::sync::Arc;

use crate::api_client::ApiClient;
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
        request: crate::dto::parking::request::GetMetricsRequest,
    ) -> ApiResult<crate::dto::parking::response::GetMetricsResponse> {
        let customer_id = request.customer_id;
        let period_start_ptz = request.period_start_ptz;
        let period_end_ptz = request.period_end_ptz;
        let limit = request.limit;
        let offset = request.offset;
        let x_request_id = request.x_request_id;
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
            .map(crate::dto::parking::response::GetMetricsResponse::from_value)
    }

    pub async fn get_metrics_by_domain(
        &self,
        request: crate::dto::parking::request::GetMetricsByDomainRequest,
    ) -> ApiResult<crate::dto::parking::response::GetMetricsByDomainResponse> {
        let customer_id = request.customer_id;
        let start_date = request.start_date;
        let end_date = request.end_date;
        let domains = request.domains;
        let domain_like = request.domain_like;
        let portfolio_id = request.portfolio_id;
        let limit = request.limit;
        let offset = request.offset;
        let x_request_id = request.x_request_id;
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
            .map(crate::dto::parking::response::GetMetricsByDomainResponse::from_value)
    }
}
