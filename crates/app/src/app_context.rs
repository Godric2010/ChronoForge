use domain::services::project_service::ProjectService;
use domain::services::report_service::ReportService;
use domain::services::task_service::TaskService;
use domain::services::time_entry_service::TimeEntryService;
use domain::services::user_settings_service::UserSettingsService;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use storage::repositories::sqlite_active_timer_repository::SqliteActiveTimerRepository;
use storage::repositories::sqlite_project_repository::SQLiteProjectRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;
use storage::repositories::sqlite_time_entry_repository::SqliteTimeEntryRepository;
use storage::repositories::sqlite_user_settings_repository::SQLiteUserSettingsRepository;

pub struct AppContext {
    pub user_settings_service: UserSettingsService<SQLiteUserSettingsRepository>,
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
        SQLiteUserSettingsRepository,
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

        let user_settings_repo = SQLiteUserSettingsRepository::new(pool.clone());
        let project_repo = SQLiteProjectRepository::new(pool.clone());
        let task_repo = SQLiteTaskRepository::new(pool.clone());
        let entry_repo = SqliteTimeEntryRepository::new(pool.clone());
        let active_repo = SqliteActiveTimerRepository::new(pool.clone());

        Ok(Self {
            user_settings_service: UserSettingsService::new(user_settings_repo.clone()),
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
                user_settings_repo.clone(),
            ),
        })
    }
}
