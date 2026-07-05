use crate::app_context::AppContext;
use clap::ArgAction;
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(clap::Subcommand)]
pub enum ProjectCommand {
    #[command(about = "Create a new project")]
    Create {
        #[arg(help = "The name of the project")]
        name: String,
    },
    #[command(about = "List projects")]
    List {
        #[arg(
            long,
            help = "Include archived projects",
            action = ArgAction::SetTrue
        )]
        include_archived: bool,
    },
    #[command(about = "Archive project")]
    Archive {
        #[arg(help = "The name of the project")]
        name: String,
    },
    #[command(about = "Unarchive project")]
    Unarchive {
        #[arg(help = "The name of the project")]
        name: String,
    },
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
            ProjectCommand::List { include_archived } => {
                let projects = app.project_service.find_all(*include_archived).await?;
                for project in projects {
                    if project.is_archived {
                        println!("[ARCHIVED] {} - {}", project.name, project.id);
                    }
                    println!("{} - {}", project.name, project.id);
                }
            }
            ProjectCommand::Archive { name } => {
                const DO_NOT_INCLUDE_ARCHIVED: bool = false;
                let projects = app
                    .project_service
                    .find_all(DO_NOT_INCLUDE_ARCHIVED)
                    .await?;
                let project = projects.iter().find(|p| p.name == *name);
                if let Some(project) = project {
                    app.project_service.archive_project(project.id).await?;
                    println!("Archived project {} ({})", project.name, project.id);
                } else {
                    println!(
                        "Could not find project with name {} in project service",
                        name
                    );
                }
            }
            ProjectCommand::Unarchive { name } => {
                const INCLUDE_ARCHIVED: bool = true;
                let projects = app.project_service.find_all(INCLUDE_ARCHIVED).await?;
                let project = projects.iter().find(|p| p.name == *name);
                if let Some(project) = project {
                    app.project_service.unarchive_project(project.id).await?;
                    println!("Unarchived project {} ({})", project.name, project.id);
                } else {
                    println!(
                        "Could not find project with name {} in project service",
                        name
                    );
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
