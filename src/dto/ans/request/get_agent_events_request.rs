use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetAgentEventsRequest {
    pub x_request_id: Option<ParamValue>,
    pub provider_id: Option<ParamValue>,
    pub last_log_id: Option<ParamValue>,
    pub limit: Option<ParamValue>,
}

impl GetAgentEventsRequest {
    pub fn new(
        x_request_id: Option<ParamValue>,
        provider_id: Option<ParamValue>,
        last_log_id: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> Self {
        Self {
            x_request_id,
            provider_id,
            last_log_id,
            limit,
        }
    }
}
