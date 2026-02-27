use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct AbuseService {
    inner: AbstractService,
}

impl AbuseService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn get_tickets(
        &self,
        type_: Option<ParamValue>,
        closed: Option<ParamValue>,
        source_domain_or_ip: Option<ParamValue>,
        target: Option<ParamValue>,
        created_start: Option<ParamValue>,
        created_end: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> ApiResult<Value> {
        self.inner
            .call(
                "GET",
                "/v1/abuse/tickets",
                Vec::new(),
                vec![
                    ("type", type_),
                    ("closed", closed),
                    ("sourceDomainOrIp", source_domain_or_ip),
                    ("target", target),
                    ("createdStart", created_start),
                    ("createdEnd", created_end),
                    ("limit", limit),
                    ("offset", offset),
                ],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn create_ticket(&self, body: impl Into<ParamValue>) -> ApiResult<Value> {
        let body = body.into();
        self.inner
            .call(
                "POST",
                "/v1/abuse/tickets",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
    }

    pub async fn get_ticket_info(&self, ticket_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let ticket_id = ticket_id.into();
        self.inner
            .call(
                "GET",
                "/v1/abuse/tickets/{ticketId}",
                vec![("ticketId", Some(ticket_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_tickets_v2(
        &self,
        type_: Option<ParamValue>,
        closed: Option<ParamValue>,
        source_domain_or_ip: Option<ParamValue>,
        target: Option<ParamValue>,
        created_start: Option<ParamValue>,
        created_end: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> ApiResult<Value> {
        self.inner
            .call(
                "GET",
                "/v2/abuse/tickets",
                Vec::new(),
                vec![
                    ("type", type_),
                    ("closed", closed),
                    ("sourceDomainOrIp", source_domain_or_ip),
                    ("target", target),
                    ("createdStart", created_start),
                    ("createdEnd", created_end),
                    ("limit", limit),
                    ("offset", offset),
                ],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn create_ticket_v2(&self, body: impl Into<ParamValue>) -> ApiResult<Value> {
        let body = body.into();
        self.inner
            .call(
                "POST",
                "/v2/abuse/tickets",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
    }

    pub async fn get_ticket_info_v2(&self, ticket_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let ticket_id = ticket_id.into();
        self.inner
            .call(
                "GET",
                "/v2/abuse/tickets/{ticketId}",
                vec![("ticketId", Some(ticket_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }
}
