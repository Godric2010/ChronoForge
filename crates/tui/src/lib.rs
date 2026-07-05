use crate::screens::overview::OverviewViewModel;
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod app;
mod app_action;
mod event;
pub mod input;
pub mod screens;
mod terminal;
mod ui_error_message;
mod widgets;

pub async fn run<B: TuiBackend>(backend: &B) -> anyhow::Result<()> {
    let mut terminal = terminal::init_terminal()?;

    let result = app::App::new().run(backend, &mut terminal).await;

    terminal::restore_terminal()?;

    result
}

#[async_trait::async_trait]
pub trait TuiBackend {
    async fn load_projects(&self) -> anyhow::Result<OverviewViewModel>;
    async fn create_project(
        &self,
        project_name: &str,
        time_limit: Option<u32>,
    ) -> anyhow::Result<()>;
    async fn delete_project(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn edit_project(
        &self,
        project_id: Uuid,
        name: &str,
        time_limit: Option<u32>,
    ) -> anyhow::Result<()>;

    async fn create_task(
        &self,
        name: String,
        time_limit: Option<u32>,
        project_id: Uuid,
    ) -> anyhow::Result<()>;
    async fn delete_task(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn edit_task(
        &self,
        task_id: Uuid,
        name: String,
        time_limit: Option<u32>,
    ) -> anyhow::Result<()>;
    async fn assign_task(&self, task_id: Uuid, project_id: Uuid) -> anyhow::Result<()>;

    async fn create_time_entry(
        &self,
        task_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> anyhow::Result<()>;
    async fn edit_time_entry(
        &self,
        entry_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> anyhow::Result<()>;
    async fn assign_time_entry(&self, entry_id: Uuid, task_id: Uuid) -> anyhow::Result<()>;
    async fn delete_time_entry(&self, entry_id: Uuid) -> anyhow::Result<()>;

    async fn get_active_time(&self) -> anyhow::Result<Option<u32>>;

    async fn start_timer(&self, task_id: Uuid) -> anyhow::Result<()>;
    async fn stop_timer(&self) -> anyhow::Result<()>;

    async fn export_csv(&self, path_str: String) -> anyhow::Result<()>;
    async fn import_csv(&self, path_str: String) -> anyhow::Result<()>;
    async fn set_show_archived_projects(&self, show_archived_projects: bool) -> anyhow::Result<()>;
    async fn set_show_archived_tasks(&self, show_archived_tasks: bool) -> anyhow::Result<()>;
}
