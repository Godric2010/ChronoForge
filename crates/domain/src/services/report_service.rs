use crate::errors::{AppError, AppResult};
use crate::repositories::active_timer_repository::ActiveTimerRepository;
use crate::repositories::project_repository::ProjectRepository;
use crate::repositories::task_repository::TaskRepository;
use crate::repositories::time_entry_repository::TimeEntryRepository;
use chrono::{DateTime, Utc};

pub struct ReportService<
    P: ProjectRepository,
    T: TaskRepository,
    E: TimeEntryRepository,
    A: ActiveTimerRepository,
> {
    project_repository: P,
    task_repository: T,
    time_entry_repository: E,
    active_timer_repository: A,
}

impl<P: ProjectRepository, T: TaskRepository, E: TimeEntryRepository, A: ActiveTimerRepository>
    ReportService<P, T, E, A>
{
    pub fn new(
        project_repository: P,
        task_repository: T,
        time_entry_repository: E,
        active_timer_repository: A,
    ) -> Self {
        Self {
            project_repository,
            task_repository,
            time_entry_repository,
            active_timer_repository,
        }
    }

    pub async fn get_active_timer_start_time(&self) -> AppResult<Option<DateTime<Utc>>> {
        let active_timer = self.active_timer_repository.get_active_timer().await;
        if active_timer.is_err() {
            return Err(AppError::Storage(format!(
                "Error when fetching active timer: {}",
                active_timer.err().unwrap()
            )));
        }
        let active_timer = active_timer.unwrap();
        if active_timer.is_none() {
            return Ok(None);
        }

        let active_timer = active_timer.unwrap();
        let start_time = active_timer.start_time;
        Ok(Some(start_time))
    }
}
