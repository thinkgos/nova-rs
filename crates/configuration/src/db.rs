use std::time::Duration;

use secrecy::SecretBox;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    url: String,                       // 连接url
    min_connections: Option<u32>,      // 默认 10
    max_connections: Option<u32>,      // 默认 500
    connect_timeout: Option<Duration>, // 默认 15s
    acquire_timeout: Option<Duration>, // 默认 10s
    idle_timeout: Option<Duration>,    // 默认 600s
    max_lifetime: Option<Duration>,    // 默认 1800s
}

impl Database {
    pub fn url(&self) -> SecretBox<str> {
        SecretBox::from(self.url.as_str())
    }
    pub fn get_min_connections(&self) -> u32 {
        self.min_connections.unwrap_or(10)
    }
    pub fn get_max_connections(&self) -> u32 {
        self.max_connections.unwrap_or(500)
    }
    pub fn get_connect_timeout(&self) -> Duration {
        self.connect_timeout.unwrap_or(Duration::from_secs(15))
    }
    pub fn get_acquire_timeout(&self) -> Duration {
        self.acquire_timeout.unwrap_or(Duration::from_secs(10))
    }
    pub fn get_idle_timeout(&self) -> Duration {
        self.idle_timeout.unwrap_or(Duration::from_secs(600))
    }
    pub fn get_max_lifetime(&self) -> Duration {
        self.max_lifetime.unwrap_or(Duration::from_secs(1800))
    }
}
