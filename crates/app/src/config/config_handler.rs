use crate::config::{AppConfig, DatabaseConfig};
use anyhow;
use directories::ProjectDirs;
use std::path::PathBuf;

const DB_FILE_NAME: &str = "chrono-forge.db";
const CONFIG_FILE_NAME: &str = "config.toml";

pub struct ConfigHandler {
    config_file: PathBuf,
    data_local_path: PathBuf,
}

impl ConfigHandler {
    pub fn new() -> anyhow::Result<Self> {
        let project_dirs = ProjectDirs::from("", "", "chrono-forge")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project dirs"))?;
        Ok(Self {
            config_file: project_dirs
                .config_dir()
                .to_path_buf()
                .join(CONFIG_FILE_NAME),
            data_local_path: project_dirs.data_local_dir().to_path_buf(),
        })
    }
    pub fn create_new_default_config(&self) -> anyhow::Result<AppConfig> {
        self.create_new_config(self.data_local_path.clone())
    }

    pub fn create_new_config(&self, path: PathBuf) -> anyhow::Result<AppConfig> {
        let config = AppConfig {
            database: DatabaseConfig {
                path: path.join(DB_FILE_NAME),
            },
        };
        self.write_config(&config)?;
        Ok(config)
    }

    pub fn write_config(&self, config: &AppConfig) -> anyhow::Result<()> {
        let parent_dir = self
            .config_file
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
        std::fs::write(self.config_file.clone(), toml_string)?;
        Ok(())
    }

    pub fn load_config(&self) -> anyhow::Result<Option<AppConfig>> {
        if !self.config_file.exists() {
            return Ok(None);
        }

        let toml_string = std::fs::read_to_string(self.config_file.clone())?;
        let config: AppConfig = toml::from_str(&toml_string)?;
        Ok(Some(config))
    }

    pub fn move_database(&self, app_config: &mut AppConfig, path: PathBuf) -> anyhow::Result<()> {
        let old_path = &app_config.database.path;
        let new_path = path.join(DB_FILE_NAME);
        std::fs::rename(old_path, &new_path)?;
        app_config.database.path = new_path;

        self.write_config(app_config)?;

        Ok(())
    }

    pub fn relink_database(&self, app_config: &mut AppConfig, path: PathBuf) -> anyhow::Result<()> {
        app_config.database.path = path;

        self.write_config(app_config)?;
        Ok(())
    }
}
