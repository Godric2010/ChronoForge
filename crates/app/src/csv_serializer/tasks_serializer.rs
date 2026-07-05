use domain::services::task_service::TaskService;
use domain::types::Task;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::path::Path;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;

#[derive(Serialize, Deserialize)]
pub struct TaskCsvRow {
    pub id: String,
    pub project_id: String,
    pub name: String,
    #[serde(default)]
    pub time_limit: Option<u32>,
    #[serde(default)]
    pub is_archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Task> for TaskCsvRow {
    fn from(value: Task) -> Self {
        Self {
            id: value.id.to_string(),
            project_id: value.project_id.to_string(),
            name: value.name,
            time_limit: value.time_limit,
            is_archived: value.is_archived,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

impl TryFrom<TaskCsvRow> for Task {
    type Error = anyhow::Error;
    fn try_from(value: TaskCsvRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            project_id: Uuid::parse_str(&value.project_id)?,
            name: value.name,
            time_limit: value.time_limit,
            is_archived: value.is_archived,
            created_at: DateTime::parse_from_rfc3339(&value.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&value.updated_at)?.with_timezone(&Utc),
        })
    }
}

pub struct TasksSerializer<'a> {
    task_service: &'a TaskService<SQLiteTaskRepository, SQLiteProjectRepository>,
}

impl<'a> TasksSerializer<'a> {
    pub fn new(
        task_service: &'a TaskService<SQLiteTaskRepository, SQLiteProjectRepository>,
    ) -> Self {
        Self { task_service }
    }

    pub async fn export_csv(&self, path: &Path) -> anyhow::Result<()> {
        let file_path = path.join("tasks.csv");

        let mut writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        let tasks = self.task_service.find_all(true).await?;
        for task in tasks {
            let row = TaskCsvRow::from(task);
            writer.serialize(row)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub async fn import_csv(&self, path: &Path) -> anyhow::Result<()> {
        let file_path = path.join("tasks.csv");
        if !file_path.exists() {
            anyhow::bail!("Missing tasks.csv file at {}", path.display());
        }

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        for result in reader.deserialize() {
            let row: TaskCsvRow = result?;
            let task = Task::try_from(row)?;
            self.task_service.upsert(task).await?;
        }
        Ok(())
    }
}
