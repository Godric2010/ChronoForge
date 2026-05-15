use clap::Parser;
use crate::app_context::AppContext;
use crate::cli::Cli;

mod app_context;
mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenvy::dotenv().ok();
    println!("Hello, world!");

    let database_url = database_url_next_to_exe()?;
    let context = AppContext::new(&database_url).await?;
    
    let cli = Cli::parse();
    cli.run(&context).await?;

    Ok(())
}

fn database_url_next_to_exe() -> anyhow::Result<String> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| anyhow::anyhow!("Could not find executable directory"))?;
    let db_path = exe_dir.join("chrono-forge.db");
    Ok(format!("sqlite:{}", db_path.display()))
}