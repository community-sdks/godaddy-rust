use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        keys: impl Into<ParamValue>,
        x_private_label_id: Option<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let keys = keys.into();
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
    }
}
