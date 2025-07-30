#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub partner_id: String,
    pub base_url: String,
    pub version: String,
    pub timeout: u64,
}

impl Config {
    pub fn new(api_key: impl Into<String>, partner_id: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            partner_id: partner_id.into(),
            base_url: "https://api.usesmileid.com".to_string(),
            version: "1.0".to_string(),
            timeout: 30,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
}
