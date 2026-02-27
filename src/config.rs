use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub base_url: Option<String>,
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub default_headers: BTreeMap<String, String>,
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            base_url: None,
            timeout: Duration::from_secs(30),
            max_retries: 2,
            retry_delay: Duration::from_millis(200),
            default_headers: BTreeMap::new(),
            user_agent: String::from("community-sdks/godaddy-rust"),
        }
    }
}
