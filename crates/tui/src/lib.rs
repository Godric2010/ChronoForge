use crate::screens::overview::OverviewViewModel;
use crate::screens::settings::settings_view_model::UserSettingsViewModel;
use crate::setup_app::{SetupApp, SetupResult};
use chrono::{DateTime, Utc, Weekday};
use domain::types::DailyTimer;
use std::path::PathBuf;
use uuid::Uuid;

mod app_action;
mod app_render_helper;
mod event;
pub mod input;
mod main_app;
pub mod screens;
pub mod setup_app;
mod terminal;
mod ui_error_message;
mod widgets;

pub async fn setup() -> anyhow::Result<SetupResult> {
    let mut terminal = terminal::init_terminal()?;

    let setup_app = SetupApp::new();
    let result = setup_app.run(&mut terminal).await;

    terminal::restore_terminal()?;

    result
}

pub async fn run<B: TuiBackend>(backend: &mut B) -> anyhow::Result<()> {
    let mut terminal = terminal::init_terminal()?;

    let mut tui_app = main_app::MainApp::new();
    let result = tui_app.run(backend, &mut terminal).await;

    terminal::restore_terminal()?;

    result
}

#[async_trait::async_trait]
pub trait TuiBackend {
    async fn load_projects(&self) -> anyhow::Result<OverviewViewModel>;
    async fn load_settings(&self) -> anyhow::Result<UserSettingsViewModel>;
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

    async fn archive_project(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn archive_task(&self, task_id: Uuid) -> anyhow::Result<()>;
    async fn unarchive_project(&self, project_id: Uuid) -> anyhow::Result<()>;
    async fn unarchive_task(&self, task_id: Uuid) -> anyhow::Result<()>;

    async fn get_daily_time(&self) -> anyhow::Result<DailyTimer>;

    async fn start_timer(&self, task_id: Uuid) -> anyhow::Result<()>;
    async fn stop_timer(&self) -> anyhow::Result<()>;

    async fn export_csv(&self, path_str: String) -> anyhow::Result<()>;
    async fn import_csv(&self, path_str: String) -> anyhow::Result<()>;
    async fn move_database(&mut self, path: PathBuf) -> anyhow::Result<()>;
    async fn relink_database(&mut self, path: PathBuf) -> anyhow::Result<()>;

    async fn set_show_archived_projects(&self, show_archived_projects: bool) -> anyhow::Result<()>;
    async fn set_show_archived_tasks(&self, show_archived_tasks: bool) -> anyhow::Result<()>;
    async fn set_workday_work_targets(
        &self,
        target_time: u32,
        weekday: Weekday,
    ) -> anyhow::Result<()>;
}
