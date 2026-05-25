use domain::services::task_service::TaskService;
use domain::types::Task;
use serde::Serialize;
use std::path::PathBuf;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;

#[derive(Serialize)]
pub struct TaskCsvRow {
    pub id: String,
    pub project_id: String,
    pub name: String,
}

impl From<Task> for TaskCsvRow {
    fn from(value: Task) -> Self {
        Self {
            id: value.id.to_string(),
            project_id: value.project_id.to_string(),
            name: value.name,
        }
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

    pub async fn export_csv(&self, path: &PathBuf) -> anyhow::Result<()> {
        let file_path = path.join("tasks.csv");

        let mut writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        let tasks = self.task_service.find_all().await?;
        for task in tasks {
            let row = TaskCsvRow::from(task);
            writer.serialize(row)?;
        }
        writer.flush()?;
        Ok(())
    }
}
