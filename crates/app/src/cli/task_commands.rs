use crate::app_context::AppContext;
use sqlx::types::Uuid;

#[derive(clap::Subcommand)]
pub enum TaskCommands {
    Create { name: String, project_id: String },
    List,
    Delete { id: String },
    Edit { id: String, name: String },
    Assign { id: String, project_id: String },
}

impl TaskCommands {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        match self {
            TaskCommands::Create { name, project_id } => {
                let project_uuid = Uuid::parse_str(&project_id)?;
                let task = app.task_service.create(name, &project_uuid).await?;
                println!("Created task {} ({})", task.name, task.id);
            }
            TaskCommands::List => {
                let tasks = app.task_service.find_all().await?;
                for task in tasks {
                    println!("{} - {}", task.name, task.id);
                }
            }
            TaskCommands::Delete { id } => {
                let uuid = Uuid::parse_str(&id)?;
                app.task_service.delete(uuid).await?;
                println!("Deleted task {}", uuid);
            }
            TaskCommands::Edit { id, name } => {
                let uuid = Uuid::parse_str(&id)?;
                let task = app.task_service.edit_task_name(uuid, name).await?;
                println!("Renamed Task {} ({})", task.name, task.id);
            }
            TaskCommands::Assign { id, project_id } => {
                let uuid = Uuid::parse_str(&id)?;
                let project_uuid = Uuid::parse_str(&project_id)?;
                let task = app
                    .task_service
                    .assign_to_project(uuid, project_uuid)
                    .await?;
                println!("Assigned Task {} to project {}", task.id, task.project_id);
            }
        }
        Ok(())
    }
}
