mod projects_serializer;
mod serialization_meta;
mod tasks_serializer;
mod time_entry_serializer;

use crate::app_context::AppContext;
use crate::csv_serializer::projects_serializer::ProjectsSerializer;
use crate::csv_serializer::serialization_meta::SerializableMeta;
use crate::csv_serializer::tasks_serializer::TasksSerializer;
use crate::csv_serializer::time_entry_serializer::TimeEntrySerializer;
use std::fs;
use std::path::{Path, PathBuf};

pub struct CsvSerializer<'a> {
    meta_serializer: SerializableMeta,
    project_serializer: ProjectsSerializer<'a>,
    tasks_serializer: TasksSerializer<'a>,
    time_entry_serializer: TimeEntrySerializer<'a>,
}

impl<'a> CsvSerializer<'a> {
    pub fn new(app_context: &'a AppContext) -> Self {
        Self {
            meta_serializer: SerializableMeta::new(),
            project_serializer: ProjectsSerializer::new(&app_context.project_service),
            tasks_serializer: TasksSerializer::new(&app_context.task_service),
            time_entry_serializer: TimeEntrySerializer::new(&app_context.time_entry_service),
        }
    }

    pub async fn export(&self, path_str: String) -> anyhow::Result<()> {
        let export_path = self.prepare_export_directory(path_str)?;
        self.meta_serializer.write_to_file(&export_path)?;
        self.project_serializer.export_csv(&export_path).await?;
        self.tasks_serializer.export_csv(&export_path).await?;
        self.time_entry_serializer.export_csv(&export_path).await?;
        Ok(())
    }

    pub async fn import(&self, path_str: String) -> anyhow::Result<()> {
        let import_path = self.prepare_import_directory(path_str)?;
        let _meta = self.meta_serializer.read_from_file(&import_path)?;
        self.project_serializer.import_csv(&import_path).await?;
        self.tasks_serializer.import_csv(&import_path).await?;
        self.time_entry_serializer.import_csv(&import_path).await?;

        Ok(())
    }

    fn prepare_export_directory(&self, path_str: String) -> anyhow::Result<PathBuf> {
        let base_path = self.expand_home_path(path_str)?;
        self.validate_path(&base_path)?;

        let export_dir = base_path.join("ChronoForge");
        fs::create_dir_all(&export_dir)?;
        Ok(export_dir)
    }

    fn prepare_import_directory(&self, path_str: String) -> anyhow::Result<PathBuf> {
        let base_path = self.expand_home_path(path_str)?;
        self.validate_path(&base_path)?;

        let meta_path = base_path.join("meta.json");

        if !meta_path.exists() {
            anyhow::bail!(
                "Missing meta.json file in import directory {}",
                base_path.display()
            );
        }

        Ok(base_path)
    }

    fn validate_path(&self, path_buf: &Path) -> anyhow::Result<()> {
        if !path_buf.exists() {
            anyhow::bail!("Path does not exist: {}", path_buf.display());
        }

        if !path_buf.is_dir() {
            anyhow::bail!("Path is not a directory: {}", path_buf.display());
        }

        Ok(())
    }

    fn expand_home_path(&self, path_str: String) -> anyhow::Result<PathBuf> {
        if let Some(stripped) = path_str.strip_prefix("~/") {
            let home_dir =
                dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot get home directory"))?;
            Ok(home_dir.join(stripped))
        } else {
            Ok(PathBuf::from(path_str))
        }
    }
}
