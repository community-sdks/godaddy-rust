use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetMetricsRequest {
    pub customer_id: ParamValue,
    pub period_start_ptz: Option<ParamValue>,
    pub period_end_ptz: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub offset: Option<ParamValue>,
    pub x_request_id: Option<ParamValue>,
}

impl GetMetricsRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        period_start_ptz: Option<ParamValue>,
        period_end_ptz: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            period_start_ptz,
            period_end_ptz,
            limit,
            offset,
            x_request_id,
        }
    }
}
