use crate::app_context::AppContext;
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(clap::Subcommand)]
pub enum ProjectCommand {
    Create { name: String },
    List,
    Delete { id: String },
    Rename { id: String, name: String },
}

impl ProjectCommand {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        match self {
            ProjectCommand::Create { name } => {
                let project = app.project_service.create(name.clone()).await?;
                println!("Created project {} ({})", project.name, project.id);
            }
            ProjectCommand::List => {
                let projects = app.project_service.find_all().await?;
                for project in projects {
                    println!("{} - {}", project.name, project.id);
                }
            }
            ProjectCommand::Delete { id } => {
                let uuid = Uuid::from_str(&id)?;
                app.project_service.delete(uuid).await?;
                println!("Deleted project {}", id);
            }
            ProjectCommand::Rename { id, name } => {
                let uuid = Uuid::from_str(&id)?;
                let project = app.project_service.edit_name(uuid, name.as_str()).await?;
                println!("Renamed project: {} ({})", project.name, project.id);
            }
        }
        Ok(())
    }
}
