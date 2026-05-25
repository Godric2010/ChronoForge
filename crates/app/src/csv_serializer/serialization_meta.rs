use std::fs::File;
use std::path::PathBuf;
use sqlx::types::chrono::Utc;

pub const EXPORT_FORMAT_VERSION: u32 = 1;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerializableMeta {
    app: String,
    version: String,
    exported_at: String,
    format_version: u32,
}

impl SerializableMeta {
    pub fn new() -> Self {
        Self {
            app: env!("CARGO_BIN_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            exported_at: Utc::now().to_rfc3339(),
            format_version: EXPORT_FORMAT_VERSION,
        }
    }

    pub fn write_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let file_path = path.join("meta.json");
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }
}
