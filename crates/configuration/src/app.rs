use serde::Deserialize;

/// 应用基本配置
#[derive(Debug, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
}

impl App {
    /// 地址, host:port
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
