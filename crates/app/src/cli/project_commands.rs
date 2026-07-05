use crate::app_context::AppContext;
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(clap::Subcommand)]
pub enum ProjectCommand {
    #[command(about = "Create a new project")]
    Create {
        #[arg(help = "The name of the project")]
        name: String,
    },
    #[command(about = "List all projects")]
    List,
    #[command(about = "Delete a project")]
    Delete {
        #[arg(help = "The id of the project that shall be deleted")]
        id: String,
    },
    #[command(about = "Rename a project")]
    Rename {
        #[arg(help = "The id of the project to rename")]
        id: String,
        #[arg(help = "The new name of the project")]
        name: String,
    },
    #[command(about = "Set the time limit of the project (in minutes)")]
    EditTimeLimit {
        #[arg(help = "The project id to edit")]
        id: String,
        #[arg(help = "The time limit (in minutes)")]
        time_limit: u32,
    },
}

impl ProjectCommand {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        match self {
            ProjectCommand::Create { name } => {
                let project = app.project_service.create(name.clone(), None).await?;
                println!("Created project {} ({})", project.name, project.id);
            }
            ProjectCommand::List => {
                let projects = app.project_service.find_all(false).await?;
                for project in projects {
                    println!("{} - {}", project.name, project.id);
                }
            }
            ProjectCommand::Delete { id } => {
                let uuid = Uuid::from_str(id)?;
                app.project_service.delete(uuid).await?;
                println!("Deleted project {}", id);
            }
            ProjectCommand::Rename { id, name } => {
                let uuid = Uuid::from_str(id)?;
                let project = app.project_service.edit_name(uuid, name.as_str()).await?;
                println!("Renamed project: {} ({})", project.name, project.id);
            }
            ProjectCommand::EditTimeLimit { id, time_limit } => {
                let uuid = Uuid::from_str(id)?;
                let time_limit = if *time_limit > 0 {
                    Some(*time_limit)
                } else {
                    None
                };
                app.project_service
                    .edit_time_limit(uuid, time_limit)
                    .await?;
                let project = app.project_service.find_by_id(uuid).await?;
                println!(
                    "Set project \"{}\" time limit to: {} min",
                    project.name,
                    project.time_limit.unwrap_or_default()
                );
            }
        }
        Ok(())
    }
}
