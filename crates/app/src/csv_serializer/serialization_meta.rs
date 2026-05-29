use sqlx::types::chrono::Utc;
use std::fs::File;
use std::path::Path;

pub const EXPORT_FORMAT_VERSION: u32 = 2;

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

    pub fn write_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let file_path = path.join("meta.json");
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }

    pub fn read_from_file(&self, path: &Path) -> anyhow::Result<Self> {
        let file_path = path.join("meta.json");
        let file = File::open(file_path)?;

        let meta: SerializableMeta = serde_json::from_reader(file)?;

        if meta.format_version != EXPORT_FORMAT_VERSION {
            anyhow::bail!("Unsupported format version: {}", meta.format_version);
        }

        Ok(meta)
    }
}
