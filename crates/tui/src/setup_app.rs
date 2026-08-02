use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;
use std::path::PathBuf;

pub enum SetupResult {
    Quit,
    CreateNewDatabase(PathBuf),
    ConnectToDatabase(PathBuf),
}

pub struct SetupApp {}

impl SetupApp {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(
        mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<SetupResult> {
        Ok(SetupResult::Quit)
    }
}
