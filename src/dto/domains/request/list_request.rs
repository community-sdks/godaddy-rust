use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub x_shopper_id: Option<ParamValue>,
    pub statuses: Option<ParamValue>,
    pub status_groups: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub marker: Option<ParamValue>,
    pub includes: Option<ParamValue>,
    pub modified_date: Option<ParamValue>,
}

impl ListRequest {
    pub fn new(
        x_shopper_id: Option<ParamValue>,
        statuses: Option<ParamValue>,
        status_groups: Option<ParamValue>,
        limit: Option<ParamValue>,
        marker: Option<ParamValue>,
        includes: Option<ParamValue>,
        modified_date: Option<ParamValue>,
    ) -> Self {
        Self {
            x_shopper_id,
            statuses,
            status_groups,
            limit,
            marker,
            includes,
            modified_date,
        }
    }
}
