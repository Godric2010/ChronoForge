use crate::app_context::AppContext;
use crate::cli::entry_commands::EntryCommands;
use crate::cli::project_commands::ProjectCommand;
use crate::cli::task_commands::TaskCommands;
use crate::cli::timer_commands::TimerCommands;

mod entry_commands;
mod project_commands;
mod task_commands;
mod timer_commands;

#[derive(clap::Parser)]
#[command(name = "chrono-forge", version, about = "A terminal based personal time tracking tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Subcommand)]
pub enum Command {
    #[command(subcommand, about = "Manage projects")]
    Project(ProjectCommand),
    #[command(subcommand, about = "Manage tasks")]
    Tasks(TaskCommands),
    #[command(subcommand, about = "Manage time entries")]
    Entry(Box<EntryCommands>),
    #[command(subcommand, about = "Start / Stop timer")]
    Timer(TimerCommands),
}

impl Cli {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        if self.command.is_none() {
            return Err(anyhow::anyhow!("No command given!"));
        };

        let command = self.command.as_ref().unwrap();
        match &command {
            Command::Project(command) => command.run(app).await,
            Command::Tasks(command) => command.run(app).await,
            Command::Entry(command) => command.run(app).await,
            Command::Timer(command) => command.run(app).await,
        }
    }
}
