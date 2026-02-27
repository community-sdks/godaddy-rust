use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        agent_display_name: Option<ParamValue>,
        version: Option<ParamValue>,
        agent_host: Option<ParamValue>,
        protocol: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> ApiResult<Value> {
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
    }

    pub async fn register_agent(&self, body: impl Into<ParamValue>) -> ApiResult<Value> {
        let body = body.into();
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
    }

    pub async fn resolve_ans_name(&self, body: impl Into<ParamValue>) -> ApiResult<Value> {
        let body = body.into();
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
    }

    pub async fn get_agent(&self, agent_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let agent_id = agent_id.into();
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
    }

    pub async fn validate_registration(&self, agent_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let agent_id = agent_id.into();
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
    }

    pub async fn verify_dns_records(&self, agent_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let agent_id = agent_id.into();
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
    }

    pub async fn get_agent_identity_certificate_by_agent_id(
        &self,
        agent_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let agent_id = agent_id.into();
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
    }

    pub async fn submit_agent_identity_csr_by_agent_id(
        &self,
        agent_id: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let agent_id = agent_id.into();
        let body = body.into();
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
    }

    pub async fn get_agent_server_certificate_by_agent_id(
        &self,
        agent_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let agent_id = agent_id.into();
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
    }

    pub async fn submit_agent_server_csr_by_agent_id(
        &self,
        agent_id: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let agent_id = agent_id.into();
        let body = body.into();
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
    }

    pub async fn get_agent_csr_status_by_agent_id(
        &self,
        agent_id: impl Into<ParamValue>,
        csr_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let agent_id = agent_id.into();
        let csr_id = csr_id.into();
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
    }

    pub async fn get_agent_events(
        &self,
        x_request_id: Option<ParamValue>,
        provider_id: Option<ParamValue>,
        last_log_id: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> ApiResult<Value> {
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
    }
}
