use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetTicketsV2Request {
    pub type_: Option<ParamValue>,
    pub closed: Option<ParamValue>,
    pub source_domain_or_ip: Option<ParamValue>,
    pub target: Option<ParamValue>,
    pub created_start: Option<ParamValue>,
    pub created_end: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub offset: Option<ParamValue>,
}

impl GetTicketsV2Request {
    pub fn new(
        type_: Option<ParamValue>,
        closed: Option<ParamValue>,
        source_domain_or_ip: Option<ParamValue>,
        target: Option<ParamValue>,
        created_start: Option<ParamValue>,
        created_end: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> Self {
        Self {
            type_,
            closed,
            source_domain_or_ip,
            target,
            created_start,
            created_end,
            limit,
            offset,
        }
    }
}
