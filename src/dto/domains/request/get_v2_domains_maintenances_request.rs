use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetV2DomainsMaintenancesRequest {
    pub x_request_id: Option<ParamValue>,
    pub status: Option<ParamValue>,
    pub modified_at_after: Option<ParamValue>,
    pub starts_at_after: Option<ParamValue>,
    pub limit: Option<ParamValue>,
}

impl GetV2DomainsMaintenancesRequest {
    pub fn new(
        x_request_id: Option<ParamValue>,
        status: Option<ParamValue>,
        modified_at_after: Option<ParamValue>,
        starts_at_after: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> Self {
        Self {
            x_request_id,
            status,
            modified_at_after,
            starts_at_after,
            limit,
        }
    }
}
