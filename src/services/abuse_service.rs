use std::sync::Arc;

use crate::api_client::ApiClient;
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
        request: crate::dto::abuse::request::GetTicketsRequest,
    ) -> ApiResult<crate::dto::abuse::response::GetTicketsResponse> {
        let type_ = request.type_;
        let closed = request.closed;
        let source_domain_or_ip = request.source_domain_or_ip;
        let target = request.target;
        let created_start = request.created_start;
        let created_end = request.created_end;
        let limit = request.limit;
        let offset = request.offset;
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
            .map(crate::dto::abuse::response::GetTicketsResponse::from_value)
    }

    pub async fn create_ticket(
        &self,
        request: crate::dto::abuse::request::CreateTicketRequest,
    ) -> ApiResult<crate::dto::abuse::response::CreateTicketResponse> {
        let body = request.body;
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
            .map(crate::dto::abuse::response::CreateTicketResponse::from_value)
    }

    pub async fn get_ticket_info(
        &self,
        request: crate::dto::abuse::request::GetTicketInfoRequest,
    ) -> ApiResult<crate::dto::abuse::response::GetTicketInfoResponse> {
        let ticket_id = request.ticket_id;
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
            .map(crate::dto::abuse::response::GetTicketInfoResponse::from_value)
    }

    pub async fn get_tickets_v2(
        &self,
        request: crate::dto::abuse::request::GetTicketsV2Request,
    ) -> ApiResult<crate::dto::abuse::response::GetTicketsV2Response> {
        let type_ = request.type_;
        let closed = request.closed;
        let source_domain_or_ip = request.source_domain_or_ip;
        let target = request.target;
        let created_start = request.created_start;
        let created_end = request.created_end;
        let limit = request.limit;
        let offset = request.offset;
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
            .map(crate::dto::abuse::response::GetTicketsV2Response::from_value)
    }

    pub async fn create_ticket_v2(
        &self,
        request: crate::dto::abuse::request::CreateTicketV2Request,
    ) -> ApiResult<crate::dto::abuse::response::CreateTicketV2Response> {
        let body = request.body;
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
            .map(crate::dto::abuse::response::CreateTicketV2Response::from_value)
    }

    pub async fn get_ticket_info_v2(
        &self,
        request: crate::dto::abuse::request::GetTicketInfoV2Request,
    ) -> ApiResult<crate::dto::abuse::response::GetTicketInfoV2Response> {
        let ticket_id = request.ticket_id;
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
            .map(crate::dto::abuse::response::GetTicketInfoV2Response::from_value)
    }
}
