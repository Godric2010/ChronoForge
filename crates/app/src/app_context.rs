use domain::services::project_service::ProjectService;
use domain::services::task_service::TaskService;
use domain::services::time_entry_service::TimeEntryService;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::types::Uuid;
use std::str::FromStr;
use storage::repositories::sqlite_active_timer_repository::SqliteActiveTimerRepository;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;
use storage::repositories::sqlite_time_entry_repository::SqliteTimeEntryRepository;
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
        })
    }
}

#[async_trait::async_trait]
impl TuiBackend for AppContext {
    async fn load_projects(
        &self,
    ) -> anyhow::Result<
        ProjectOverviewViewModel,
    > {
        let all_projects = self.project_service.find_all().await?;
        Ok(ProjectOverviewViewModel {
            projects: all_projects,
        })
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
}
