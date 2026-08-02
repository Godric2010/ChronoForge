use crate::app_context::AppContext;
use crate::app_view_context::AppViewContext;
use crate::cli::Cli;
use crate::config::config_handler::ConfigHandler;
use crate::config::AppConfig;
use anyhow::anyhow;
use clap::Parser;
use std::path::PathBuf;

mod app_context;
mod app_error;
mod app_view_context;
mod cli;
pub mod config;
mod csv_serializer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ConfigHandler::new()?.load_or_create_app_config()?;

    let db_path_str = config.database.path.to_str();
    if db_path_str.is_none() {
        return Err(anyhow::anyhow!("Failed to convert db path to str"));
    };
    let db_path_str = db_path_str.unwrap();

    let cli = Cli::parse();
    match cli.command {
        Some(_) => {
            run_cli_tool(cli, db_path_str).await?;
        }
        None => {
            run_tui_tool(db_path_str, &config).await?;
        }
    }

    Ok(())
}

async fn run_cli_tool(cli: Cli, db_path_str: &str) -> anyhow::Result<()> {
    let app_context = create_app_context(db_path_str).await?;
    match app_context {
        None => Err(anyhow!("Database not found!\n{}", db_path_str)),
        Some(app_context) => {
            cli.run(&app_context).await?;
            Ok(())
        }
    }
}

async fn run_tui_tool(db_path_str: &str, config: &AppConfig) -> anyhow::Result<()> {
    let app_context = create_app_context(db_path_str).await?;
    match app_context {
        None => Err(anyhow!(
            "No valid app context available! Aborting execution"
        )),
        Some(app_context) => {
            run_tui_main(app_context, config).await?;
            Ok(())
        }
    }
}

async fn run_tui_main(app_context: AppContext, config: &AppConfig) -> anyhow::Result<()> {
    let mut view_context = AppViewContext::new(&app_context, config.clone());
    tui::run(&mut view_context).await?;
    Ok(())
}

async fn create_app_context(db_path_str: &str) -> anyhow::Result<Option<AppContext>> {
    let db_path_buf = PathBuf::from(db_path_str);
    if !db_path_buf.exists() {
        return Ok(None);
    }

    let context = AppContext::new(db_path_str).await?;
    Ok(Some(context))
}
