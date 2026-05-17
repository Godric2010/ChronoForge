use uuid::Uuid;
use crate::screens::project_overview::ProjectOverviewViewModel;

mod app;
pub mod screens;
mod terminal;
mod widgets;
mod event;
mod app_action;

pub async fn run<B: TuiBackend>(backend: &B) -> anyhow::Result<()> {
    let mut terminal = terminal::init_terminal()?;

    let result = app::App::new().run(backend, &mut terminal).await;
    
    terminal::restore_terminal()?;
    
    result
}

#[async_trait::async_trait]
pub trait TuiBackend{
    async fn load_projects(&self) -> anyhow::Result<ProjectOverviewViewModel>;
    async fn create_project(&self, project_name: &str) -> anyhow::Result<()>;
    async fn delete_project(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn rename_project(&self, project_id: Uuid, name: &str) -> anyhow::Result<()>;
}
