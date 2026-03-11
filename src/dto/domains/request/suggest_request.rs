use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct SuggestRequest {
    pub x_shopper_id: Option<ParamValue>,
    pub query: Option<ParamValue>,
    pub country: Option<ParamValue>,
    pub city: Option<ParamValue>,
    pub sources: Option<ParamValue>,
    pub tlds: Option<ParamValue>,
    pub length_max: Option<ParamValue>,
    pub length_min: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub wait_ms: Option<ParamValue>,
}

impl SuggestRequest {
    pub fn new(
        x_shopper_id: Option<ParamValue>,
        query: Option<ParamValue>,
        country: Option<ParamValue>,
        city: Option<ParamValue>,
        sources: Option<ParamValue>,
        tlds: Option<ParamValue>,
        length_max: Option<ParamValue>,
        length_min: Option<ParamValue>,
        limit: Option<ParamValue>,
        wait_ms: Option<ParamValue>,
    ) -> Self {
        Self {
            x_shopper_id,
            query,
            country,
            city,
            sources,
            tlds,
            length_max,
            length_min,
            limit,
            wait_ms,
        }
    }
}
