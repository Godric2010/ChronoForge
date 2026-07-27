use crate::app_context::AppContext;
use crate::app_view_context::AppViewContext;
use crate::cli::Cli;
use crate::config_handler::config_handler::ConfigHandler;
use clap::Parser;

mod app_context;
mod app_error;
mod app_view_context;
mod cli;
pub mod config_handler;
mod csv_serializer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ConfigHandler::new()?.load_or_create_app_config()?;

    let db_path_str = config.database.path.to_str();
    if db_path_str.is_none() {
        return Err(anyhow::anyhow!("Failed to convert db path to str"));
    };
    let context = AppContext::new(db_path_str.unwrap()).await?;

    let cli = Cli::parse();
    match cli.command {
        Some(_) => {
            cli.run(&context).await?;
        }
        None => {
            let mut view_context = AppViewContext::new(&context, config);
            tui::run(&mut view_context).await?;
        }
    }

    Ok(())
}
