use crate::app_context::AppContext;
use crate::app_view_context::AppViewContext;
use crate::cli::Cli;
use clap::Parser;

mod app_context;
mod app_view_context;
mod cli;
mod csv_serializer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = database_url_next_to_exe()?;
    let context = AppContext::new(&database_url).await?;

    dbg!(std::env::args().collect::<Vec<_>>());
    let cli = Cli::parse();
    match cli.command {
        Some(_) => {
            cli.run(&context).await?;
        }
        None => {
            let view_context = AppViewContext::new(&context);
            tui::run(&view_context).await?;
        }
    }

    Ok(())
}

fn database_url_next_to_exe() -> anyhow::Result<String> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Could not find executable directory"))?;
    let db_path = exe_dir.join("chrono-forge.db");
    Ok(format!("sqlite:{}", db_path.display()))
}
