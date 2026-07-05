use crate::app_context::AppContext;
use clap::ArgAction;
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
    List {
        #[arg(long, help = "Include archived tasks", action = ArgAction::SetTrue)]
        include_archived: bool,
    },
    #[command(about = "Archive a task")]
    Archive {
        #[arg(help = "The name of the task to archive")]
        name: String,
    },
    #[command(about = "Unarchive a task")]
    Unarchive {
        #[arg(help = "The name of the task to unarchive")]
        name: String,
    },
    #[command(about = "Delete a task")]
    Delete {
        #[arg(help = "The id of the task to delete")]
        id: String,
    },
    #[command(about = "Set a new task name")]
    Rename {
        #[arg(help = "The id of the task to edit")]
        id: String,
        #[arg(help = "The new task name")]
        name: String,
    },
    #[command(about = "Set a time limit for the task (in minutes)")]
    EditTimeLimit {
        #[arg(help = "The id of the task to edit")]
        id: String,
        #[arg(help = "The time limit (in minutes)")]
        time_limit: u32,
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
                let project_uuid = Uuid::parse_str(project_id)?;
                let task = app.task_service.create(name, &project_uuid, None).await?;
                println!("Created task {} ({})", task.name, task.id);
            }
            TaskCommands::List { include_archived } => {
                let tasks = app.task_service.find_all(*include_archived).await?;
                for task in tasks {
                    if task.is_archived {
                        println!("[ARCHIVED] {} - {}", task.name, task.id);
                    }
                    println!("{} - {}", task.name, task.id);
                }
            }
            TaskCommands::Archive { name } => {
                const DO_NOT_INCLUDE_ARCHIVED: bool = false;
                let tasks = app.task_service.find_all(DO_NOT_INCLUDE_ARCHIVED).await?;
                let task = tasks.iter().find(|task| task.name == *name);
                if let Some(task) = task {
                    app.task_service.archive_task(task.id).await?;
                    println!("Archived task {} ({})", task.name, task.id);
                } else {
                    println!("Could not find task {}", name);
                }
            }
            TaskCommands::Unarchive { name } => {
                const INCLUDE_ARCHIVED: bool = true;
                let tasks = app.task_service.find_all(INCLUDE_ARCHIVED).await?;
                let task = tasks.iter().find(|task| task.name == *name);
                if let Some(task) = task {
                    app.task_service.unarchive_task(task.id).await?;
                    println!("Unarchived task {} ({})", task.name, task.id);
                } else {
                    println!("Could not find task {}", name);
                }
            }

            TaskCommands::Delete { id } => {
                let uuid = Uuid::parse_str(id)?;
                app.task_service.delete(uuid).await?;
                println!("Deleted task {}", uuid);
            }
            TaskCommands::Rename { id, name } => {
                let uuid = Uuid::parse_str(id)?;
                let task = app.task_service.edit_task_name(uuid, name).await?;
                println!("Renamed Task {} ({})", task.name, task.id);
            }
            TaskCommands::Assign { id, project_id } => {
                let uuid = Uuid::parse_str(id)?;
                let project_uuid = Uuid::parse_str(project_id)?;
                let task = app
                    .task_service
                    .assign_to_project(uuid, project_uuid)
                    .await?;
                println!("Assigned Task {} to project {}", task.id, task.project_id);
            }
            TaskCommands::EditTimeLimit { id, time_limit } => {
                let uuid = Uuid::parse_str(id)?;
                let time_limit = if *time_limit > 0 {
                    Some(*time_limit)
                } else {
                    None
                };
                app.task_service.edit_time_limit(uuid, time_limit).await?;
                let task = app.task_service.find_by_id(uuid).await?;
                println!(
                    "Edited time limit of task \"{}\". ({} min)",
                    task.name,
                    task.time_limit.unwrap_or(0)
                );
            }
        }
        Ok(())
    }
}
