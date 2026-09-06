use serde::{Deserialize, Serialize};

/// Base client configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Base URL of the default relay, e.g. `http://localhost:8080`.
    pub relay_url: String,
    /// Default `limit` used in scan requests (MIP-03 default is 20).
    pub default_limit: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            relay_url: "http://localhost:8080".to_string(),
            default_limit: 20,
        }
    }
}

impl Config {
    pub fn new(relay_url: impl Into<String>) -> Self {
        Self {
            relay_url: relay_url.into(),
            ..Self::default()
        }
    }
}
