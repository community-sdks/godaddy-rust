use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
use crate::error::ApiResult;

#[derive(Clone)]
pub struct AbstractService {
    client: Arc<ApiClient>,
    base_url: &'static str,
}

impl AbstractService {
    pub fn new(client: Arc<ApiClient>, base_url: &'static str) -> Self {
        Self { client, base_url }
    }

    pub async fn call(
        &self,
        method: &str,
        path: &str,
        path_params: Vec<(&'static str, Option<ParamValue>)>,
        query_params: Vec<(&'static str, Option<ParamValue>)>,
        headers: Vec<(&'static str, Option<ParamValue>)>,
        body: Option<ParamValue>,
    ) -> ApiResult<Value> {
        self.client
            .request(
                method,
                self.base_url,
                path,
                path_params,
                query_params,
                headers,
                body,
            )
            .await
    }
}
