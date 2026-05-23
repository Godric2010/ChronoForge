use domain::services::project_service::ProjectService;
use domain::services::report_service::ReportService;
use domain::services::task_service::TaskService;
use domain::services::time_entry_service::TimeEntryService;
use domain::types::{Project, Task, TimeEntry};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::str::FromStr;
use storage::repositories::sqlite_active_timer_repository::SqliteActiveTimerRepository;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;
use storage::repositories::sqlite_time_entry_repository::SqliteTimeEntryRepository;
use tui::screens::project_overview::project_overview_view_model::{
    ProjectViewModel, TaskViewModel, TimeEntryViewModel,
};
use tui::screens::project_overview::ProjectOverviewViewModel;
use tui::TuiBackend;

pub struct AppContext {
    pub project_service: ProjectService<SQLiteProjectRepository>,
    pub task_service: TaskService<SQLiteTaskRepository, SQLiteProjectRepository>,
    pub time_entry_service: TimeEntryService<
        SQLiteTaskRepository,
        SqliteTimeEntryRepository,
        SqliteActiveTimerRepository,
    >,
    pub report_service: ReportService<
        SQLiteProjectRepository,
        SQLiteTaskRepository,
        SqliteTimeEntryRepository,
        SqliteActiveTimerRepository,
    >,
}

impl AppContext {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        sqlx::migrate!("../../migrations").run(&pool).await?;

        let project_repo = SQLiteProjectRepository::new(pool.clone());
        let task_repo = SQLiteTaskRepository::new(pool.clone());
        let entry_repo = SqliteTimeEntryRepository::new(pool.clone());
        let active_repo = SqliteActiveTimerRepository::new(pool.clone());

        Ok(Self {
            project_service: ProjectService::new(project_repo.clone()),
            task_service: TaskService::new(task_repo.clone(), project_repo.clone()),
            time_entry_service: TimeEntryService::new(
                task_repo.clone(),
                entry_repo.clone(),
                active_repo.clone(),
            ),
            report_service: ReportService::new(
                project_repo.clone(),
                task_repo.clone(),
                entry_repo.clone(),
                active_repo.clone(),
            ),
        })
    }
}

impl AppContext {
    async fn create_overview_view_model(&self) -> anyhow::Result<ProjectOverviewViewModel> {
        let all_projects = self.project_service.find_all().await?;
        let mut projects = Vec::<ProjectViewModel>::new();
        for project in all_projects {
            let project_vm = self.create_project_view_model(&project).await?;
            projects.push(project_vm);
        }

        Ok(ProjectOverviewViewModel { projects })
    }

    async fn create_project_view_model(
        &self,
        project: &Project,
    ) -> anyhow::Result<ProjectViewModel> {
        let tasks = self.task_service.find_by_project_id(project.id).await?;

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
        sorted_entries.sort_by(|a, b| b.end_time.cmp(&a.end_time));

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
            start_time: time_entry.start_time.clone(),
            end_time: time_entry.end_time.clone(),
        })
    }
}

#[async_trait::async_trait]
impl TuiBackend for AppContext {
    async fn load_projects(&self) -> anyhow::Result<ProjectOverviewViewModel> {
        self.create_overview_view_model().await
    }

    async fn create_project(&self, project_name: &str) -> anyhow::Result<()> {
        self.project_service
            .create(project_name.to_string())
            .await?;
        Ok(())
    }

    async fn delete_project(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.project_service.delete(project_id).await?;
        Ok(())
    }

    async fn rename_project(&self, project_id: Uuid, name: &str) -> anyhow::Result<()> {
        self.project_service.edit_name(project_id, name).await?;
        Ok(())
    }

    async fn create_task(&self, name: String, project_id: Uuid) -> anyhow::Result<()> {
        self.task_service.create(&name, &project_id).await?;
        Ok(())
    }

    async fn delete_task(&self, project_id: Uuid) -> anyhow::Result<()> {
        self.task_service.delete(project_id).await?;
        Ok(())
    }

    async fn rename_task(&self, task_id: Uuid, name: String) -> anyhow::Result<()> {
        self.task_service.edit_task_name(task_id, &name).await?;
        Ok(())
    }

    async fn create_time_entry(
        &self,
        task_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        self.time_entry_service
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
        self.time_entry_service
            .edit_time_entry(entry_id, start_time, end_time)
            .await?;
        Ok(())
    }

    async fn delete_time_entry(&self, entry_id: Uuid) -> anyhow::Result<()> {
        self.time_entry_service.delete(entry_id).await?;
        Ok(())
    }

    async fn get_active_time(&self) -> anyhow::Result<Option<u32>> {
        let active_time = self.report_service.get_active_timer_start_time().await?;
        if let Some(active_time) = active_time {
            let current = Utc::now();
            let time_delta = current - active_time;
            return Ok(Some(time_delta.num_minutes() as u32));
        }
        Ok(None)
    }

    async fn start_timer(&self, task_id: Uuid) -> anyhow::Result<()> {
        self.time_entry_service.start_timer(task_id).await?;
        Ok(())
    }

    async fn stop_timer(&self) -> anyhow::Result<()> {
        self.time_entry_service.stop_timer().await?;
        Ok(())
    }
}
