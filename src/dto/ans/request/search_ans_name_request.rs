use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct SearchAnsNameRequest {
    pub agent_display_name: Option<ParamValue>,
    pub version: Option<ParamValue>,
    pub agent_host: Option<ParamValue>,
    pub protocol: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub offset: Option<ParamValue>,
}

impl SearchAnsNameRequest {
    pub fn new(
        agent_display_name: Option<ParamValue>,
        version: Option<ParamValue>,
        agent_host: Option<ParamValue>,
        protocol: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> Self {
        Self {
            agent_display_name,
            version,
            agent_host,
            protocol,
            limit,
            offset,
        }
    }
}
