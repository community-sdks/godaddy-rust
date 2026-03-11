use std::sync::Arc;

use crate::api_client::ApiClient;
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

    pub async fn create_subaccount(
        &self,
        request: crate::dto::shoppers::request::CreateSubaccountRequest,
    ) -> ApiResult<crate::dto::shoppers::response::CreateSubaccountResponse> {
        let subaccount = request.subaccount;
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
            .map(crate::dto::shoppers::response::CreateSubaccountResponse::from_value)
    }

    pub async fn get(
        &self,
        request: crate::dto::shoppers::request::GetRequest,
    ) -> ApiResult<crate::dto::shoppers::response::GetResponse> {
        let shopper_id = request.shopper_id;
        let includes = request.includes;
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
            .map(crate::dto::shoppers::response::GetResponse::from_value)
    }

    pub async fn update(
        &self,
        request: crate::dto::shoppers::request::UpdateRequest,
    ) -> ApiResult<crate::dto::shoppers::response::UpdateResponse> {
        let shopper_id = request.shopper_id;
        let shopper = request.shopper;
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
            .map(crate::dto::shoppers::response::UpdateResponse::from_value)
    }

    pub async fn delete(
        &self,
        request: crate::dto::shoppers::request::DeleteRequest,
    ) -> ApiResult<crate::dto::shoppers::response::DeleteResponse> {
        let shopper_id = request.shopper_id;
        let audit_client_ip = request.audit_client_ip;
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
            .map(crate::dto::shoppers::response::DeleteResponse::from_value)
    }

    pub async fn get_status(
        &self,
        request: crate::dto::shoppers::request::GetStatusRequest,
    ) -> ApiResult<crate::dto::shoppers::response::GetStatusResponse> {
        let shopper_id = request.shopper_id;
        let audit_client_ip = request.audit_client_ip;
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
            .map(crate::dto::shoppers::response::GetStatusResponse::from_value)
    }

    pub async fn change_password(
        &self,
        request: crate::dto::shoppers::request::ChangePasswordRequest,
    ) -> ApiResult<crate::dto::shoppers::response::ChangePasswordResponse> {
        let shopper_id = request.shopper_id;
        let secret = request.secret;
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
            .map(crate::dto::shoppers::response::ChangePasswordResponse::from_value)
    }
}
