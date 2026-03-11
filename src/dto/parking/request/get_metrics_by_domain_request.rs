use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetMetricsByDomainRequest {
    pub customer_id: ParamValue,
    pub start_date: ParamValue,
    pub end_date: ParamValue,
    pub domains: Option<ParamValue>,
    pub domain_like: Option<ParamValue>,
    pub portfolio_id: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub offset: Option<ParamValue>,
    pub x_request_id: Option<ParamValue>,
}

impl GetMetricsByDomainRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        start_date: impl Into<ParamValue>,
        end_date: impl Into<ParamValue>,
        domains: Option<ParamValue>,
        domain_like: Option<ParamValue>,
        portfolio_id: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            start_date: start_date.into(),
            end_date: end_date.into(),
            domains,
            domain_like,
            portfolio_id,
            limit,
            offset,
            x_request_id,
        }
    }
}
