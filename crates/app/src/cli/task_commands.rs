use crate::app_context::AppContext;
use sqlx::types::Uuid;

#[derive(clap::Subcommand)]
pub enum TaskCommands {
    #[command(about = "Create a new task in a project")]
    Create {
        #[arg(help = "The name of the task to create")]
        name: String,
        #[arg(help = "The id of the project this task belongs to")]
        project_id: String,
    },
    #[command(about = "List all tasks")]
    List,
    #[command(about = "Delete a task")]
    Delete {
        #[arg(help = "The id of the task to delete")]
        id: String,
    },
    #[command(about = "Set a new task name")]
    Edit {
        #[arg(help = "The id of the task to edit")]
        id: String,
        #[arg(help = "The new task name")]
        name: String,
    },
    #[command(about = "Assign a task to a new project")]
    Assign {
        #[arg(help = "The id of the  task to re-assign")]
        id: String,
        #[arg(help = "The id of the project this task shall be assigned to")]
        project_id: String,
    },
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
