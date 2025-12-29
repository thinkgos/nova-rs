mod app;
mod db;

pub use self::app::App;
pub use self::db::Database;

use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;

use core::deploy::Deploy;

/// 配置
#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub app: App,
    pub database: Database,
}

impl Configuration {
    pub fn load() -> Result<Self, anyhow::Error> {
        let deploy: Deploy = env::var("APP_DEPLOY_MODE")
            .unwrap_or_else(|_| "dev".into())
            .parse()?;
        let work_dir = env::current_dir()?;
        let config_dir = work_dir.join("conf");

        let c = Config::builder()
            // 基础配置
            .add_source(File::from(config_dir.join("app")))
            // 布署配置
            .add_source(File::from(config_dir.join(deploy.to_string())))
            // 环境变量配置
            // 环境变量格式: APP_XX.YY, 其中 `APP_` 是前缀, `XX`是属性, `YY`为属性字段
            // 例如端口: APP_APP.PORT=9999
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("."),
            )
            .build()?
            .try_deserialize()?;

        log::debug!("{:?}", c);
        Ok(c)
    }
}
