use crate::config::{AppConfig, DatabaseConfig};
use anyhow;
use directories::ProjectDirs;
use std::path::PathBuf;

const DB_FILE_NAME: &str = "chrono-forge.db";
pub struct ConfigHandler {
    config_path: PathBuf,
    data_local_path: PathBuf,
}

impl ConfigHandler {
    pub fn new() -> anyhow::Result<Self> {
        let project_dirs = ProjectDirs::from("", "", "chrono-forge")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project dirs"))?;
        Ok(Self {
            config_path: project_dirs.config_dir().to_path_buf(),
            data_local_path: project_dirs.data_local_dir().to_path_buf(),
        })
    }
    pub fn load_or_create_app_config(&self) -> anyhow::Result<AppConfig> {
        let config_path = self.config_path.join("config.toml");

        let default_database_path = self.data_local_path.join(DB_FILE_NAME);

        if config_path.exists() {
            return self.read_config(&config_path);
        }

        let config = AppConfig {
            database: DatabaseConfig {
                path: default_database_path,
            },
        };
        self.write_config(&config_path, &config)?;
        Ok(config)
    }

    pub fn write_config(&self, config_path: &PathBuf, config: &AppConfig) -> anyhow::Result<()> {
        let parent_dir = config_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Config path has no parent directory"))?;
        std::fs::create_dir_all(parent_dir)?;

        if !config.database.path.exists() {
            let db_parent = config
                .database
                .path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Database path has no parent"))?;
            std::fs::create_dir_all(db_parent)?;
        }

        let toml_string = toml::to_string(config)?;
        std::fs::write(config_path, toml_string)?;
        Ok(())
    }

    pub fn read_config(&self, config_path: &PathBuf) -> anyhow::Result<AppConfig> {
        let toml_string = std::fs::read_to_string(config_path)?;
        let config: AppConfig = toml::from_str(&toml_string)?;
        Ok(config)
    }

    pub fn move_database(&self, app_config: &mut AppConfig, path: PathBuf) -> anyhow::Result<()> {
        let old_path = &app_config.database.path;
        let new_path = path.join(DB_FILE_NAME);
        std::fs::rename(old_path, &new_path)?;
        app_config.database.path = new_path;

        let config_path = self.config_path.join("config.toml");
        self.write_config(&config_path, app_config)?;

        Ok(())
    }

    pub fn relink_database(&self, app_config: &mut AppConfig, path: PathBuf) -> anyhow::Result<()> {
        app_config.database.path = path;

        let config_path = self.config_path.join("config.toml");
        self.write_config(&config_path, app_config)?;
        Ok(())
    }
}
