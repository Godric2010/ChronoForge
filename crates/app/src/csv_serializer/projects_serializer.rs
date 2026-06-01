use domain::services::project_service::ProjectService;
use domain::types::Project;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::path::Path;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;

#[derive(Serialize, Deserialize)]
pub struct ProjectCsvRow {
    pub id: String,
    pub name: String,
    pub time_limit: u32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Project> for ProjectCsvRow {
    fn from(value: Project) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            time_limit: value.time_limit.unwrap_or_default(),
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

impl TryFrom<ProjectCsvRow> for Project {
    type Error = anyhow::Error;

    fn try_from(value: ProjectCsvRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            name: value.name,
            time_limit: if value.time_limit > 0 {
                Some(value.time_limit)
            } else {
                None
            },
            created_at: DateTime::parse_from_rfc3339(&value.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&value.updated_at)?.with_timezone(&Utc),
        })
    }
}

pub struct ProjectsSerializer<'a> {
    project_service: &'a ProjectService<SQLiteProjectRepository>,
}

impl<'a> ProjectsSerializer<'a> {
    pub fn new(project_service: &'a ProjectService<SQLiteProjectRepository>) -> Self {
        Self { project_service }
    }

    pub async fn export_csv(&self, path: &Path) -> anyhow::Result<()> {
        let file_path = path.join("projects.csv");

        let mut writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        let projects = self.project_service.find_all().await?;
        for project in projects {
            let row = ProjectCsvRow::from(project);
            writer.serialize(row)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub async fn import_csv(&self, path: &Path) -> anyhow::Result<()> {
        let file_path = path.join("projects.csv");
        if !file_path.exists() {
            anyhow::bail!("Missing projects.csv file at {}", path.display());
        }

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        for result in reader.deserialize() {
            let row: ProjectCsvRow = result?;
            let project = Project::try_from(row)?;
            self.project_service.upsert(project).await?;
        }

        Ok(())
    }
}
