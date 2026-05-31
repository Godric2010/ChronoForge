use crate::app_context::AppContext;
use crate::app_error::AppError;
use crate::csv_serializer::CsvSerializer;
use domain::types::{Project, Task, TimeEntry};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use tui::screens::overview::overview_view_model::{
    ProjectViewModel, TaskViewModel, TimeEntryViewModel,
};
use tui::screens::overview::OverviewViewModel;
use tui::TuiBackend;

pub struct AppViewContext<'a> {
    app: &'a AppContext,
}

impl<'a> AppViewContext<'a> {
    pub fn new(app: &'a AppContext) -> Self {
        Self { app }
    }

    async fn create_overview_view_model(&self) -> anyhow::Result<OverviewViewModel> {
        let all_projects = self.app.project_service.find_all().await?;
        let mut projects = Vec::<ProjectViewModel>::new();
        for project in all_projects {
            let project_vm = self.create_project_view_model(&project).await?;
            projects.push(project_vm);
        }

        Ok(OverviewViewModel { projects })
    }

    async fn create_project_view_model(
        &self,
        project: &Project,
    ) -> anyhow::Result<ProjectViewModel> {
        let tasks = self.app.task_service.find_by_project_id(project.id).await?;

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

    async fn create_project(&self, project_name: &str) -> anyhow::Result<()> {
        self.app
            .project_service
            .create(project_name.to_string())
            .await?;
        Ok(())
    }

    async fn delete_project(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.app.project_service.delete(project_id).await?;
        Ok(())
    }

    async fn rename_project(&self, project_id: Uuid, name: &str) -> anyhow::Result<()> {
        self.app.project_service.edit_name(project_id, name).await?;
        Ok(())
    }

    async fn create_task(&self, name: String, project_id: Uuid) -> anyhow::Result<()> {
        self.app.task_service.create(&name, &project_id).await?;
        Ok(())
    }

    async fn delete_task(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.app.task_service.delete(project_id).await?;
        Ok(())
    }

    async fn rename_task(&self, task_id: Uuid, name: String) -> anyhow::Result<()> {
        self.app.task_service.edit_task_name(task_id, &name).await?;
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

    async fn get_active_time(&self) -> anyhow::Result<Option<u32>> {
        let active_time = self
            .app
            .report_service
            .get_active_timer_start_time()
            .await?;
        if let Some(active_time) = active_time {
            let current = Utc::now();
            let time_delta = current - active_time;
            return Ok(Some(time_delta.num_minutes() as u32));
        }
        Ok(None)
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

    async fn export_csv(&self, path_str: String) -> anyhow::Result<()> {
        CsvSerializer::new(self.app)
            .export(path_str.clone())
            .await
            .map_err(|source| AppError::CsvExportFailed {
                path: path_str,
                source,
            })?;
        Ok(())
    }

    async fn import_csv(&self, path_str: String) -> anyhow::Result<()> {
        CsvSerializer::new(self.app)
            .import(path_str.clone())
            .await
            .map_err(|source| AppError::CsvImportFailed {
                path: path_str,
                source,
            })?;
        Ok(())
    }
}
