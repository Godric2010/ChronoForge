use crate::app_context::AppContext;
use crate::app_error::AppError;
use crate::config::config_handler::ConfigHandler;
use crate::config::AppConfig;
use crate::csv_serializer::CsvSerializer;
use chrono::Weekday;
use domain::types::{DailyTimer, Project, Task, TimeEntry};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::path::PathBuf;
use tui::screens::overview::overview_view_model::{
    ProjectViewModel, TaskViewModel, TimeEntryViewModel,
};
use tui::screens::overview::OverviewViewModel;
use tui::screens::settings::settings_view_model::UserSettingsViewModel;
use tui::TuiBackend;

pub struct AppViewContext<'a> {
    app: &'a AppContext,
    app_config: AppConfig,
}

impl<'a> AppViewContext<'a> {
    pub fn new(app: &'a AppContext, app_config: AppConfig) -> Self {
        Self { app, app_config }
    }

    async fn create_overview_view_model(&self) -> anyhow::Result<OverviewViewModel> {
        let settings = self.app.user_settings_service.get_settings().await?;
        let all_projects = self
            .app
            .project_service
            .find_all(settings.show_archived_projects)
            .await?;
        let mut projects = Vec::<ProjectViewModel>::new();
        for project in all_projects {
            let project_vm = self
                .create_project_view_model(&project, settings.show_archived_tasks)
                .await?;
            projects.push(project_vm);
        }

        Ok(OverviewViewModel { projects })
    }

    async fn create_project_view_model(
        &self,
        project: &Project,
        show_archived_tasks: bool,
    ) -> anyhow::Result<ProjectViewModel> {
        let tasks = self
            .app
            .task_service
            .find_by_project_id(project.id, show_archived_tasks)
            .await?;

        let mut project_time_minutes = 0;
        let mut task_vms = Vec::<TaskViewModel>::new();
        for task in tasks {
            let task_vm = self.create_task_view_model(&task).await?;
            project_time_minutes += task_vm.total_task_time_min;
            task_vms.push(task_vm);
        }

        Ok(ProjectViewModel {
            project: project.clone(),
            total_project_time_min: project_time_minutes,
            time_limit: project.time_limit,
            is_archived: project.is_archived,
            tasks: task_vms,
        })
    }

    async fn create_task_view_model(&self, task: &Task) -> anyhow::Result<TaskViewModel> {
        let time_entries = self
            .app
            .time_entry_service
            .find_all_entries_of_task(task.id)
            .await?;

        let mut task_time_minutes = 0;
        let mut time_entry_vms = Vec::<TimeEntryViewModel>::new();

        for time_entry in time_entries {
            let time_entry_vm = self.create_time_entry_view_model(&time_entry)?;
            task_time_minutes +=
                (time_entry_vm.end_time - time_entry_vm.start_time).num_minutes() as u32;
            time_entry_vms.push(time_entry_vm);
        }

        let mut sorted_entries = time_entry_vms.clone();
        sorted_entries.sort_by_key(|b| std::cmp::Reverse(b.end_time));

        Ok(TaskViewModel {
            task: task.clone(),
            total_task_time_min: task_time_minutes,
            time_limit: task.time_limit,
            is_archived: task.is_archived,
            time_entries: sorted_entries,
        })
    }

    fn create_time_entry_view_model(
        &self,
        time_entry: &TimeEntry,
    ) -> anyhow::Result<TimeEntryViewModel> {
        Ok(TimeEntryViewModel {
            time_entry: time_entry.clone(),
            start_time: time_entry.start_time,
            end_time: time_entry.end_time,
        })
    }
}

#[async_trait::async_trait]
impl<'a> TuiBackend for AppViewContext<'a> {
    async fn load_projects(&self) -> anyhow::Result<OverviewViewModel> {
        self.create_overview_view_model().await
    }

    async fn load_settings(&self) -> anyhow::Result<UserSettingsViewModel> {
        let user_settings = self.app.user_settings_service.get_settings().await?;
        Ok(UserSettingsViewModel {
            db_path: self.app_config.database.path.clone(),
            user_settings,
        })
    }

    async fn create_project(
        &self,
        project_name: &str,
        time_limit: Option<u32>,
    ) -> anyhow::Result<()> {
        self.app
            .project_service
            .create(project_name.to_string(), time_limit)
            .await?;
        Ok(())
    }

    async fn delete_project(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.app.project_service.delete(project_id).await?;
        Ok(())
    }

    async fn edit_project(
        &self,
        project_id: Uuid,
        name: &str,
        time_limit: Option<u32>,
    ) -> anyhow::Result<()> {
        self.app.project_service.edit_name(project_id, name).await?;
        self.app
            .project_service
            .edit_time_limit(project_id, time_limit)
            .await?;
        Ok(())
    }

    async fn create_task(
        &self,
        name: String,
        time_limit: Option<u32>,
        project_id: Uuid,
    ) -> anyhow::Result<()> {
        self.app
            .task_service
            .create(&name, &project_id, time_limit)
            .await?;
        Ok(())
    }

    async fn delete_task(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.app.task_service.delete(project_id).await?;
        Ok(())
    }

    async fn edit_task(
        &self,
        task_id: Uuid,
        name: String,
        time_limit: Option<u32>,
    ) -> anyhow::Result<()> {
        self.app.task_service.edit_task_name(task_id, &name).await?;
        self.app
            .task_service
            .edit_time_limit(task_id, time_limit)
            .await?;
        Ok(())
    }

    async fn assign_task(&self, task_id: Uuid, project_id: Uuid) -> anyhow::Result<()> {
        self.app
            .task_service
            .assign_to_project(task_id, project_id)
            .await?;
        Ok(())
    }

    async fn create_time_entry(
        &self,
        task_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        self.app
            .time_entry_service
            .create_manual(start_time, end_time, task_id)
            .await?;
        Ok(())
    }

    async fn edit_time_entry(
        &self,
        entry_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        self.app
            .time_entry_service
            .edit_time_entry(entry_id, start_time, end_time)
            .await?;
        Ok(())
    }

    async fn assign_time_entry(&self, entry_id: Uuid, task_id: Uuid) -> anyhow::Result<()> {
        self.app
            .time_entry_service
            .assign_time_entry(entry_id, task_id)
            .await?;
        Ok(())
    }

    async fn delete_time_entry(&self, entry_id: Uuid) -> anyhow::Result<()> {
        self.app.time_entry_service.delete(entry_id).await?;
        Ok(())
    }

    async fn archive_project(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.app.project_service.archive_project(project_id).await?;
        Ok(())
    }

    async fn archive_task(&self, task_id: Uuid) -> anyhow::Result<()> {
        self.app.task_service.archive_task(task_id).await?;
        Ok(())
    }

    async fn unarchive_project(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.app
            .project_service
            .unarchive_project(project_id)
            .await?;
        Ok(())
    }

    async fn unarchive_task(&self, task_id: Uuid) -> anyhow::Result<()> {
        self.app.task_service.unarchive_task(task_id).await?;
        Ok(())
    }

    async fn get_daily_time(&self) -> anyhow::Result<DailyTimer> {
        let daily_time = self.app.report_service.get_daily_work_time().await?;
        Ok(daily_time)
    }

    async fn start_timer(&self, task_id: Uuid) -> anyhow::Result<()> {
        self.app
            .time_entry_service
            .start_timer(task_id)
            .await
            .map_err(|_| AppError::IllegalAction {
                action: "Cannot start timer!".to_string(),
            })?;
        Ok(())
    }

    async fn stop_timer(&self) -> anyhow::Result<()> {
        self.app
            .time_entry_service
            .stop_timer()
            .await
            .map_err(|_| AppError::IllegalAction {
                action: "Cannot stop timer!".to_string(),
            })?;
        Ok(())
    }

    async fn export_csv(&self, path: PathBuf) -> anyhow::Result<()> {
        CsvSerializer::new(self.app)
            .export(path.clone())
            .await
            .map_err(|source| AppError::CsvExportFailed {
                path: path.to_str().unwrap_or_default().to_string(),
                source,
            })?;
        Ok(())
    }

    async fn import_csv(&self, path: PathBuf) -> anyhow::Result<()> {
        CsvSerializer::new(self.app)
            .import(path.clone())
            .await
            .map_err(|source| AppError::CsvImportFailed {
                path: path.to_str().unwrap_or_default().to_string(),
                source,
            })?;
        Ok(())
    }

    async fn move_database(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let config_handler = ConfigHandler::new()?;
        config_handler.move_database(&mut self.app_config, path)?;
        Ok(())
    }

    async fn relink_database(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let config_handler = ConfigHandler::new()?;
        config_handler.relink_database(&mut self.app_config, path)?;
        Ok(())
    }

    async fn set_show_archived_projects(&self, show_archived_projects: bool) -> anyhow::Result<()> {
        self.app
            .user_settings_service
            .set_show_archived_projects(show_archived_projects)
            .await?;
        Ok(())
    }

    async fn set_show_archived_tasks(&self, show_archived_tasks: bool) -> anyhow::Result<()> {
        self.app
            .user_settings_service
            .set_show_archived_tasks(show_archived_tasks)
            .await?;
        Ok(())
    }

    async fn set_workday_work_targets(
        &self,
        target_time: u32,
        weekday: Weekday,
    ) -> anyhow::Result<()> {
        self.app
            .user_settings_service
            .set_work_target(target_time, weekday)
            .await?;
        Ok(())
    }
}
