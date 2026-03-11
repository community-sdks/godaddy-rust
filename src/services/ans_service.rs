use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct AnsService {
    inner: AbstractService,
}

impl AnsService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn search_ans_name(
        &self,
        request: crate::dto::ans::request::SearchAnsNameRequest,
    ) -> ApiResult<crate::dto::ans::response::SearchAnsNameResponse> {
        let agent_display_name = request.agent_display_name;
        let version = request.version;
        let agent_host = request.agent_host;
        let protocol = request.protocol;
        let limit = request.limit;
        let offset = request.offset;
        self.inner
            .call(
                "GET",
                "/v1/agents",
                Vec::new(),
                vec![
                    ("agentDisplayName", agent_display_name),
                    ("version", version),
                    ("agentHost", agent_host),
                    ("protocol", protocol),
                    ("limit", limit),
                    ("offset", offset),
                ],
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::ans::response::SearchAnsNameResponse::from_value)
    }

    pub async fn register_agent(
        &self,
        request: crate::dto::ans::request::RegisterAgentRequest,
    ) -> ApiResult<crate::dto::ans::response::RegisterAgentResponse> {
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v1/agents/register",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::ans::response::RegisterAgentResponse::from_value)
    }

    pub async fn resolve_ans_name(
        &self,
        request: crate::dto::ans::request::ResolveAnsNameRequest,
    ) -> ApiResult<crate::dto::ans::response::ResolveAnsNameResponse> {
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v1/agents/resolution",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::ans::response::ResolveAnsNameResponse::from_value)
    }

    pub async fn get_agent(
        &self,
        request: crate::dto::ans::request::GetAgentRequest,
    ) -> ApiResult<crate::dto::ans::response::GetAgentResponse> {
        let agent_id = request.agent_id;
        self.inner
            .call(
                "GET",
                "/v1/agents/{agentId}",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::ans::response::GetAgentResponse::from_value)
    }

    pub async fn validate_registration(
        &self,
        request: crate::dto::ans::request::ValidateRegistrationRequest,
    ) -> ApiResult<crate::dto::ans::response::ValidateRegistrationResponse> {
        let agent_id = request.agent_id;
        self.inner
            .call(
                "POST",
                "/v1/agents/{agentId}/verify-acme",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::ans::response::ValidateRegistrationResponse::from_value)
    }

    pub async fn verify_dns_records(
        &self,
        request: crate::dto::ans::request::VerifyDnsRecordsRequest,
    ) -> ApiResult<crate::dto::ans::response::VerifyDnsRecordsResponse> {
        let agent_id = request.agent_id;
        self.inner
            .call(
                "POST",
                "/v1/agents/{agentId}/verify-dns",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::ans::response::VerifyDnsRecordsResponse::from_value)
    }

    pub async fn get_agent_identity_certificate_by_agent_id(
        &self,
        request: crate::dto::ans::request::GetAgentIdentityCertificateByAgentIdRequest,
    ) -> ApiResult<crate::dto::ans::response::GetAgentIdentityCertificateByAgentIdResponse> {
        let agent_id = request.agent_id;
        self.inner
            .call(
                "GET",
                "/v1/agents/{agentId}/certificates/identity",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(
                crate::dto::ans::response::GetAgentIdentityCertificateByAgentIdResponse::from_value,
            )
    }

    pub async fn submit_agent_identity_csr_by_agent_id(
        &self,
        request: crate::dto::ans::request::SubmitAgentIdentityCsrByAgentIdRequest,
    ) -> ApiResult<crate::dto::ans::response::SubmitAgentIdentityCsrByAgentIdResponse> {
        let agent_id = request.agent_id;
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v1/agents/{agentId}/certificates/identity",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::ans::response::SubmitAgentIdentityCsrByAgentIdResponse::from_value)
    }

    pub async fn get_agent_server_certificate_by_agent_id(
        &self,
        request: crate::dto::ans::request::GetAgentServerCertificateByAgentIdRequest,
    ) -> ApiResult<crate::dto::ans::response::GetAgentServerCertificateByAgentIdResponse> {
        let agent_id = request.agent_id;
        self.inner
            .call(
                "GET",
                "/v1/agents/{agentId}/certificates/server",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::ans::response::GetAgentServerCertificateByAgentIdResponse::from_value)
    }

    pub async fn submit_agent_server_csr_by_agent_id(
        &self,
        request: crate::dto::ans::request::SubmitAgentServerCsrByAgentIdRequest,
    ) -> ApiResult<crate::dto::ans::response::SubmitAgentServerCsrByAgentIdResponse> {
        let agent_id = request.agent_id;
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v1/agents/{agentId}/certificates/server",
                vec![("agentId", Some(agent_id))],
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::ans::response::SubmitAgentServerCsrByAgentIdResponse::from_value)
    }

    pub async fn get_agent_csr_status_by_agent_id(
        &self,
        request: crate::dto::ans::request::GetAgentCsrStatusByAgentIdRequest,
    ) -> ApiResult<crate::dto::ans::response::GetAgentCsrStatusByAgentIdResponse> {
        let agent_id = request.agent_id;
        let csr_id = request.csr_id;
        self.inner
            .call(
                "GET",
                "/v1/agents/{agentId}/csrs/{csrId}/status",
                vec![("agentId", Some(agent_id)), ("csrId", Some(csr_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::ans::response::GetAgentCsrStatusByAgentIdResponse::from_value)
    }

    pub async fn get_agent_events(
        &self,
        request: crate::dto::ans::request::GetAgentEventsRequest,
    ) -> ApiResult<crate::dto::ans::response::GetAgentEventsResponse> {
        let x_request_id = request.x_request_id;
        let provider_id = request.provider_id;
        let last_log_id = request.last_log_id;
        let limit = request.limit;
        self.inner
            .call(
                "GET",
                "/v1/agents/events",
                Vec::new(),
                vec![
                    ("providerId", provider_id),
                    ("lastLogId", last_log_id),
                    ("limit", limit),
                ],
                vec![("X-Request-Id", x_request_id)],
                None,
            )
            .await
            .map(crate::dto::ans::response::GetAgentEventsResponse::from_value)
    }
}
