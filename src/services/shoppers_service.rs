use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct ShoppersService {
    inner: AbstractService,
}

impl ShoppersService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn create_subaccount(&self, subaccount: impl Into<ParamValue>) -> ApiResult<Value> {
        let subaccount = subaccount.into();
        self.inner
            .call(
                "POST",
                "/v1/shoppers/subaccount",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(subaccount),
            )
            .await
    }

    pub async fn get(
        &self,
        shopper_id: impl Into<ParamValue>,
        includes: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let shopper_id = shopper_id.into();
        self.inner
            .call(
                "GET",
                "/v1/shoppers/{shopperId}",
                vec![("shopperId", Some(shopper_id))],
                vec![("includes", includes)],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn update(
        &self,
        shopper_id: impl Into<ParamValue>,
        shopper: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let shopper_id = shopper_id.into();
        let shopper = shopper.into();
        self.inner
            .call(
                "POST",
                "/v1/shoppers/{shopperId}",
                vec![("shopperId", Some(shopper_id))],
                Vec::new(),
                Vec::new(),
                Some(shopper),
            )
            .await
    }

    pub async fn delete(
        &self,
        shopper_id: impl Into<ParamValue>,
        audit_client_ip: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let shopper_id = shopper_id.into();
        let audit_client_ip = audit_client_ip.into();
        self.inner
            .call(
                "DELETE",
                "/v1/shoppers/{shopperId}",
                vec![("shopperId", Some(shopper_id))],
                vec![("auditClientIp", Some(audit_client_ip))],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_status(
        &self,
        shopper_id: impl Into<ParamValue>,
        audit_client_ip: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let shopper_id = shopper_id.into();
        let audit_client_ip = audit_client_ip.into();
        self.inner
            .call(
                "GET",
                "/v1/shoppers/{shopperId}/status",
                vec![("shopperId", Some(shopper_id))],
                vec![("auditClientIp", Some(audit_client_ip))],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn change_password(
        &self,
        shopper_id: impl Into<ParamValue>,
        secret: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let shopper_id = shopper_id.into();
        let secret = secret.into();
        self.inner
            .call(
                "PUT",
                "/v1/shoppers/{shopperId}/factors/password",
                vec![("shopperId", Some(shopper_id))],
                Vec::new(),
                Vec::new(),
                Some(secret),
            )
            .await
    }
}
