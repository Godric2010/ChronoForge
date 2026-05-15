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
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Project(ProjectCommand),
    #[command(subcommand)]
    Tasks(TaskCommands),
    #[command(subcommand)]
    Entry(EntryCommands),
    #[command(subcommand)]
    Timer(TimerCommands),
}

impl Cli {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        match &self.command {
            Command::Project(command) => command.run(app).await,
            Command::Tasks(command) => command.run(app).await,
            Command::Entry(command) => command.run(app).await,
            Command::Timer(command) => command.run(app).await,
        }
    }
}
