use crate::app_context::AppContext;
use crate::app_view_context::AppViewContext;
use crate::cli::Cli;
use crate::config::config_handler::ConfigHandler;
use crate::config::AppConfig;
use anyhow::anyhow;
use clap::Parser;
use std::path::PathBuf;
use tui::setup_app::SetupResult;

mod app_context;
mod app_error;
mod app_view_context;
mod cli;
pub mod config;
mod csv_serializer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_handler = ConfigHandler::new()?;
    let cli = Cli::parse();
    match cli.command {
        Some(_) => {
            run_cli_tool(cli, &config_handler).await?;
        }
        None => {
            run_tui_tool(&config_handler).await?;
        }
    }

    Ok(())
}

async fn run_cli_tool(cli: Cli, config_handler: &ConfigHandler) -> anyhow::Result<()> {
    let mut create_new_db: bool = false;
    let app_config = match config_handler.load_config()? {
        None => {
            create_new_db = true;
            config_handler.create_new_default_config()?
        }
        Some(config) => config,
    };

    let db_path = app_config.database.path;

    let app_context = create_app_context(&db_path, create_new_db).await?;
    cli.run(&app_context).await?;
    Ok(())
}

async fn run_tui_tool(config_handler: &ConfigHandler) -> anyhow::Result<()> {
    let app_config = config_handler.load_config()?;
    match app_config {
        None => {
            run_tui_setup(&config_handler).await?;
            Ok(())
        }
        Some(app_config) => {
            let database_exists = app_config.database_exists();
            if !database_exists {
                run_tui_setup(&config_handler).await?;
                return Ok(());
            }
            let app_context = create_app_context(&app_config.database.path, false).await?;
            run_tui_main(app_context, &app_config).await?;
            Ok(())
        }
    }
}

async fn run_tui_setup(config_handler: &ConfigHandler) -> anyhow::Result<()> {
    let setup_result = tui::setup().await?;
    match setup_result {
        SetupResult::Quit => Ok(()),
        SetupResult::CreateNewDatabase(path) => {
            let config = config_handler.create_new_config(path)?;
            let app_context = create_app_context(&config.database.path, true).await?;
            run_tui_main(app_context, &config).await?;
            Ok(())
        }
        SetupResult::ConnectToDatabase(path) => {
            let config = config_handler.create_new_config(path)?;
            if !config.database_exists() {
                return Err(anyhow!("Failed to create new config/database"));
            }
            let app_context = create_app_context(&config.database.path, false).await?;
            run_tui_main(app_context, &config).await?;
            Ok(())
        }
    }
}

async fn run_tui_main(app_context: AppContext, config: &AppConfig) -> anyhow::Result<()> {
    let mut view_context = AppViewContext::new(&app_context, config.clone());
    tui::run(&mut view_context).await?;
    Ok(())
}

async fn create_app_context(db_path: &PathBuf, create_new_db: bool) -> anyhow::Result<AppContext> {
    let db_path_str = db_path
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert db path to str"))?;

    let context = AppContext::new(db_path_str, create_new_db).await?;
    Ok(context)
}
