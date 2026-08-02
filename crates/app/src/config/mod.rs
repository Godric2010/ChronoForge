use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod config_handler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: PathBuf,
}

impl AppConfig {
    pub fn database_exists(&self) -> bool {
        self.database.path.exists()
    }
}
