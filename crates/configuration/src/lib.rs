mod app;
mod configuration;
mod db;

pub use crate::app::App;
pub use crate::configuration::Configuration;
pub use crate::db::Database;

use std::sync::OnceLock;

static CONFIG: OnceLock<Configuration> = OnceLock::new();

pub fn try_init() -> Result<(), anyhow::Error> {
    let c = Configuration::try_init()?;
    CONFIG.set(c).unwrap();

    Ok(())
}

pub fn use_config() -> &'static Configuration {
    let Some(c) = CONFIG.get() else {
        panic!("Configuration not initialized");
    };
    c
}
