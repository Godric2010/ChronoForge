use crate::screens::project_overview::ProjectOverviewViewModel;
use uuid::Uuid;

mod app;
mod app_action;
mod event;
pub mod screens;
mod terminal;
mod widgets;

pub async fn run<B: TuiBackend>(backend: &B) -> anyhow::Result<()> {
    let mut terminal = terminal::init_terminal()?;

    let result = app::App::new().run(backend, &mut terminal).await;

    terminal::restore_terminal()?;

    result
}

#[async_trait::async_trait]
pub trait TuiBackend {
    async fn load_projects(&self) -> anyhow::Result<ProjectOverviewViewModel>;
    async fn create_project(&self, project_name: &str) -> anyhow::Result<()>;
    async fn delete_project(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn rename_project(&self, project_id: Uuid, name: &str) -> anyhow::Result<()>;

    async fn create_task(&self, name: String, project_id: Uuid) -> anyhow::Result<()>;
    async fn delete_task(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn rename_task(&self, task_id: Uuid, name: String) -> anyhow::Result<()>;

    async fn get_active_time(&self) -> anyhow::Result<Option<u32>>;

    async fn start_timer(&self, task_id: Uuid) -> anyhow::Result<()>;
    async fn stop_timer(&self) -> anyhow::Result<()>;
}
