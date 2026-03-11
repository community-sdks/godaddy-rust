use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetV2DomainsUsageYyyymmRequest {
    pub yyyymm: ParamValue,
    pub x_request_id: Option<ParamValue>,
    pub includes: Option<ParamValue>,
}

impl GetV2DomainsUsageYyyymmRequest {
    pub fn new(
        yyyymm: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
        includes: Option<ParamValue>,
    ) -> Self {
        Self {
            yyyymm: yyyymm.into(),
            x_request_id,
            includes,
        }
    }
}
