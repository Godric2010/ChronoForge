use domain::services::project_service::ProjectService;
use domain::types::Project;
use serde::Serialize;
use std::path::PathBuf;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;

#[derive(Serialize)]
pub struct ProjectCsvRow {
    pub id: String,
    pub name: String,
}

impl From<Project> for ProjectCsvRow {
    fn from(value: Project) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
        }
    }
}

pub struct ProjectsSerializer<'a> {
    project_service: &'a ProjectService<SQLiteProjectRepository>,
}

impl<'a> ProjectsSerializer<'a> {
    pub fn new(project_service: &'a ProjectService<SQLiteProjectRepository>) -> Self {
        Self { project_service }
    }

    pub async fn export_csv(&self, path: &PathBuf) -> anyhow::Result<()> {
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
}
