use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct AgreementsService {
    inner: AbstractService,
}

impl AgreementsService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn get(
        &self,
        request: crate::dto::agreements::request::GetRequest,
    ) -> ApiResult<crate::dto::agreements::response::GetResponse> {
        let keys = request.keys;
        let x_private_label_id = request.x_private_label_id;
        let x_market_id = request.x_market_id;
        self.inner
            .call(
                "GET",
                "/v1/agreements",
                Vec::new(),
                vec![("keys", Some(keys))],
                vec![
                    ("X-Private-Label-Id", x_private_label_id),
                    ("X-Market-Id", x_market_id),
                ],
                None,
            )
            .await
            .map(crate::dto::agreements::response::GetResponse::from_value)
    }
}
