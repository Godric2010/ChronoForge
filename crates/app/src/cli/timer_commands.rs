use crate::app_context::AppContext;
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(clap::Subcommand)]
pub enum TimerCommands {
    #[command(about = "Start a timer for a task")]
    Start {
        #[arg(help = "The id of the task this timer works on")]
        task_id: String,
    },
    #[command(about = "Stop a timer")]
    Stop,
}

impl TimerCommands {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        match self {
            TimerCommands::Start { task_id } => {
                let task_uuid = Uuid::from_str(task_id.as_str())?;
                app.time_entry_service.start_timer(task_uuid).await?;
                println!("Started timer task {}", task_uuid);
            }
            TimerCommands::Stop => {
                app.time_entry_service.stop_timer().await?;
                println!("Stopped timer");
            }
        }
        Ok(())
    }
}
