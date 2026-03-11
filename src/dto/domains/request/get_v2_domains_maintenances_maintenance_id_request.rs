use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetV2DomainsMaintenancesMaintenanceIdRequest {
    pub maintenance_id: ParamValue,
    pub x_request_id: Option<ParamValue>,
}

impl GetV2DomainsMaintenancesMaintenanceIdRequest {
    pub fn new(maintenance_id: impl Into<ParamValue>, x_request_id: Option<ParamValue>) -> Self {
        Self {
            maintenance_id: maintenance_id.into(),
            x_request_id,
        }
    }
}
