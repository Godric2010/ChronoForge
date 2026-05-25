use crate::errors::{AppError, AppResult};
use crate::repositories::active_timer_repository::ActiveTimerRepository;
use crate::repositories::task_repository::TaskRepository;
use crate::repositories::time_entry_repository::TimeEntryRepository;
use crate::types::{ActiveTimer, TimeEntry};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TimeEntryService<T: TaskRepository, E: TimeEntryRepository, A: ActiveTimerRepository> {
    task_repository: T,
    time_entry_repository: E,
    active_timer_repository: A,
}

impl<T: TaskRepository, E: TimeEntryRepository, A: ActiveTimerRepository>
    TimeEntryService<T, E, A>
{
    pub fn new(task_repository: T, time_entry_repository: E, active_timer_repository: A) -> Self {
        Self {
            task_repository,
            time_entry_repository,
            active_timer_repository,
        }
    }

    pub async fn create_manual(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        task_id: Uuid,
    ) -> AppResult<TimeEntry> {
        self.is_task_id_valid(&task_id).await?;

        if start_time >= end_time {
            return Err(AppError::InvalidTimeRange);
        }

        let time_entry = TimeEntry {
            id: Uuid::new_v4(),
            task_id,
            start_time,
            end_time,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.time_entry_repository.create(time_entry.clone()).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(time_entry)
    }

    pub async fn start_timer(&self, task_id: Uuid) -> AppResult<ActiveTimer> {
        let active_timer = self.active_timer_repository.get_active_timer().await;
        if active_timer.is_err() {
            return Err(AppError::Storage(active_timer.unwrap_err().to_string()));
        }
        if active_timer.unwrap().is_some() {
            return Err(AppError::TimerAlreadyRunning);
        }

        self.is_task_id_valid(&task_id).await?;

        let active_timer = ActiveTimer {
            task_id,
            start_time: Utc::now(),
        };
        let result = self.active_timer_repository.set(active_timer.clone()).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(active_timer)
    }

    pub async fn stop_timer(&self) -> AppResult<TimeEntry> {
        let active_timer = self.active_timer_repository.get_active_timer().await;
        if active_timer.is_err() {
            return Err(AppError::Storage(active_timer.unwrap_err().to_string()));
        }
        let active_timer = active_timer.unwrap();
        if active_timer.is_none() {
            return Err(AppError::NoActiveTimer);
        }

        let active_timer = active_timer.unwrap();
        let time_entry = TimeEntry {
            task_id: active_timer.task_id,
            id: Uuid::new_v4(),
            start_time: active_timer.start_time,
            end_time: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.time_entry_repository.create(time_entry.clone()).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        let result = self.active_timer_repository.remove().await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(time_entry)
    }

    pub async fn delete(&self, entry_id: Uuid) -> AppResult<()> {
        self.get_by_id(entry_id).await?;
        let result = self.time_entry_repository.delete(entry_id).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(())
    }

    pub async fn find_all(&self) -> AppResult<Vec<TimeEntry>> {
        let all_entries = self.time_entry_repository.find_all().await;
        if all_entries.is_err() {
            return Err(AppError::Storage(all_entries.unwrap_err().to_string()));
        }
        Ok(all_entries.unwrap())
    }

    pub async fn find_all_entries_of_task(&self, task_id: Uuid) -> AppResult<Vec<TimeEntry>> {
        self.is_task_id_valid(&task_id).await?;
        let result = self.time_entry_repository.find_by_task_id(task_id).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(result.unwrap())
    }

    pub async fn edit_time_entry(
        &self,
        entry_id: Uuid,
        new_start_time: DateTime<Utc>,
        new_end_time: DateTime<Utc>,
    ) -> AppResult<TimeEntry> {
        if new_end_time < new_start_time {
            return Err(AppError::InvalidTimeRange);
        }

        let time_entry = self.get_by_id(entry_id).await?;

        let modified_time_entry = TimeEntry {
            id: time_entry.id,
            task_id: time_entry.task_id,
            start_time: new_start_time,
            end_time: new_end_time,
            created_at: time_entry.created_at,
            updated_at: Utc::now(),
        };
        let result = self
            .time_entry_repository
            .update(modified_time_entry.clone())
            .await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(modified_time_entry)
    }

    pub async fn assign_time_entry(
        &self,
        entry_id: Uuid,
        new_task_id: Uuid,
    ) -> AppResult<TimeEntry> {
        self.is_task_id_valid(&new_task_id).await?;

        let time_entry = self.get_by_id(entry_id).await?;
        let modified_time_entry = TimeEntry {
            id: time_entry.id,
            task_id: new_task_id,
            start_time: time_entry.start_time,
            end_time: time_entry.end_time,
            created_at: time_entry.created_at,
            updated_at: Utc::now(),
        };
        let result = self
            .time_entry_repository
            .update(modified_time_entry.clone())
            .await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        Ok(modified_time_entry)
    }

    async fn get_by_id(&self, entry_id: Uuid) -> AppResult<TimeEntry> {
        let result = self.time_entry_repository.find_by_id(entry_id).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        let result = result.unwrap();
        if result.is_none() {
            return Err(AppError::TimeEntryNotFound);
        }
        Ok(result.unwrap())
    }
    async fn is_task_id_valid(&self, task_id: &Uuid) -> AppResult<()> {
        let result = self.task_repository.find_by_id(task_id).await;
        if result.is_err() {
            return Err(AppError::Storage(result.unwrap_err().to_string()));
        }
        let result = result.unwrap();
        match result {
            Some(_) => Ok(()),
            None => Err(AppError::TaskNotFound),
        }
    }
}

#[cfg(test)]
mod time_entry_service_tests {
    use super::*;
    use crate::services::project_service::ProjectService;
    use crate::services::task_service::TaskService;
    use crate::test_support::in_memory_active_timer_repository::InMemoryActiveTimerRepository;
    use crate::test_support::in_memory_project_repository::InMemoryProjectRepository;
    use crate::test_support::in_memory_task_repository::InMemoryTaskRepository;
    use crate::test_support::in_memory_time_entry_repository::InMemoryTimeEntryRepository;
    use chrono::{TimeZone, Timelike};

    struct Context {
        service: TimeEntryService<
            InMemoryTaskRepository,
            InMemoryTimeEntryRepository,
            InMemoryActiveTimerRepository,
        >,
        task_ids: Vec<Uuid>,
    }

    impl Context {
        pub async fn new() -> Self {
            let project_repo = InMemoryProjectRepository::new();
            let task_repo = InMemoryTaskRepository::new();
            let time_entry_repo = InMemoryTimeEntryRepository::new();
            let active_timer_repo = InMemoryActiveTimerRepository::new();

            let service =
                TimeEntryService::new(task_repo.clone(), time_entry_repo, active_timer_repo);

            let project_service = ProjectService::new(project_repo.clone());
            let task_service = TaskService::new(task_repo.clone(), project_repo.clone());

            let project_01 = project_service
                .create("Project01".to_string())
                .await
                .unwrap();
            let project_02 = project_service
                .create("Project02".to_string())
                .await
                .unwrap();

            let task_01 = task_service.create("Task01", &project_01.id).await.unwrap();
            let task_02 = task_service.create("Task02", &project_02.id).await.unwrap();
            let task_03 = task_service.create("Task03", &project_01.id).await.unwrap();
            let task_04 = task_service.create("Task04", &project_02.id).await.unwrap();

            let task_ids = vec![task_01.id, task_02.id, task_03.id, task_04.id];
            Self { service, task_ids }
        }
    }

    fn utc(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
        let utc = Utc
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .unwrap();
        utc
    }
    #[tokio::test]
    async fn create_manual_entry() {
        let context = Context::new().await;

        let start_time = utc(2026, 05, 08, 22, 05);
        let end_time = utc(2026, 05, 08, 22, 07);
        let result = context
            .service
            .create_manual(start_time, end_time, context.task_ids[0])
            .await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.start_time, start_time);
        assert_eq!(result.end_time, end_time);
        assert_ne!(result.id, Uuid::default());
        assert_eq!(result.task_id, context.task_ids[0]);
    }

    #[tokio::test]
    async fn create_time_entry_with_invalid_time() {
        let context = Context::new().await;

        let end_time = utc(2026, 05, 08, 22, 05);
        let start_time = utc(2026, 05, 08, 22, 07);
        let result = context
            .service
            .create_manual(start_time, end_time, context.task_ids[0])
            .await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::InvalidTimeRange))
    }

    #[tokio::test]
    async fn create_time_entry_with_invalid_task() {
        let context = Context::new().await;

        let end_time = utc(2026, 05, 08, 22, 05);
        let start_time = utc(2026, 05, 08, 22, 07);
        let result = context
            .service
            .create_manual(start_time, end_time, Uuid::new_v4())
            .await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::TaskNotFound))
    }

    #[tokio::test]
    async fn start_timer() {
        let context = Context::new().await;

        let result = context.service.start_timer(context.task_ids[0]).await;
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.task_id, context.task_ids[0]);
        assert!(result.start_time.nanosecond() > 0);

        assert!(context
            .service
            .active_timer_repository
            .get_active_timer()
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn start_timer_while_another_is_already_started() {
        let context = Context::new().await;

        let result = context.service.start_timer(context.task_ids[0]).await;
        assert!(result.is_ok());

        let second_result = context.service.start_timer(context.task_ids[1]).await;
        assert!(second_result.is_err());
        assert!(matches!(
            second_result.err().unwrap(),
            AppError::TimerAlreadyRunning
        ))
    }

    #[tokio::test]
    async fn start_timer_with_invalid_task_id() {
        let context = Context::new().await;
        let result = context.service.start_timer(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::TaskNotFound))
    }

    #[tokio::test]
    async fn stop_timer_with_active_timer_running() {
        let context = Context::new().await;

        let active = ActiveTimer {
            task_id: context.task_ids[0],
            start_time: utc(2000, 05, 08, 22, 05),
        };
        context
            .service
            .active_timer_repository
            .set(active.clone())
            .await
            .unwrap();

        let result = context.service.stop_timer().await;
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.task_id, active.task_id);
        assert_eq!(result.start_time, active.start_time);
        assert!(result.end_time > active.start_time);
        assert_ne!(result.id, Uuid::default());

        assert!(context
            .service
            .active_timer_repository
            .get_active_timer()
            .await
            .unwrap()
            .is_none())
    }

    #[tokio::test]
    async fn stop_timer_with_no_active_timer_running() {
        let context = Context::new().await;

        let result = context.service.stop_timer().await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::NoActiveTimer))
    }

    #[tokio::test]
    async fn delete_time_entry_with_valid_id() {
        let context = Context::new().await;
        let time_entry = context
            .service
            .create_manual(
                utc(2026, 05, 09, 14, 10),
                utc(2026, 05, 09, 14, 13),
                context.task_ids[0],
            )
            .await
            .unwrap();

        let result = context.service.delete(time_entry.id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_time_entry_with_invalid_id() {
        let context = Context::new().await;

        let result = context.service.delete(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::TimeEntryNotFound))
    }

    #[tokio::test]
    async fn find_time_entries_by_valid_task_id() {
        let context = Context::new().await;
        context
            .service
            .create_manual(
                utc(2026, 05, 09, 14, 10),
                utc(2026, 05, 09, 14, 13),
                context.task_ids[0],
            )
            .await
            .unwrap();
        context
            .service
            .create_manual(
                utc(2026, 05, 09, 14, 10),
                utc(2026, 05, 09, 14, 13),
                context.task_ids[0],
            )
            .await
            .unwrap();
        context
            .service
            .create_manual(
                utc(2026, 05, 09, 14, 10),
                utc(2026, 05, 09, 14, 13),
                context.task_ids[1],
            )
            .await
            .unwrap();

        let expected_entries = 2;
        let result = context
            .service
            .find_all_entries_of_task(context.task_ids[0])
            .await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), expected_entries);
    }

    #[tokio::test]
    async fn find_time_entries_by_invalid_task_id() {
        let context = Context::new().await;
        let result = context
            .service
            .find_all_entries_of_task(Uuid::new_v4())
            .await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::TaskNotFound))
    }

    #[tokio::test]
    async fn edit_time_entry() {
        let context = Context::new().await;

        let original_start_time = utc(2026, 05, 09, 14, 10);
        let original_end_time = utc(2026, 05, 09, 14, 13);
        let original_time_entry = context
            .service
            .create_manual(original_start_time, original_end_time, context.task_ids[0])
            .await
            .unwrap();

        let modified_start_time = utc(2025, 05, 09, 14, 10);
        let modified_end_time = utc(2025, 05, 09, 15, 13);

        let modified_time_entry = context
            .service
            .edit_time_entry(
                original_time_entry.id,
                modified_start_time,
                modified_end_time,
            )
            .await;
        assert!(modified_time_entry.is_ok());
        let modified_time_entry = modified_time_entry.unwrap();
        assert_eq!(modified_time_entry.start_time, modified_start_time);
        assert_eq!(modified_time_entry.end_time, modified_end_time);
        assert_eq!(modified_time_entry.id, original_time_entry.id);
        assert_eq!(modified_time_entry.task_id, original_time_entry.task_id);
    }

    #[tokio::test]
    async fn edit_time_entry_with_invalid_entry_id() {
        let context = Context::new().await;
        let modified_start_time = utc(2025, 05, 09, 14, 10);
        let modified_end_time = utc(2025, 05, 09, 15, 13);

        let modified_time_entry = context
            .service
            .edit_time_entry(Uuid::new_v4(), modified_start_time, modified_end_time)
            .await;
        assert!(modified_time_entry.is_err());
        assert!(matches!(
            modified_time_entry.err().unwrap(),
            AppError::TimeEntryNotFound
        ))
    }

    #[tokio::test]
    async fn edit_time_entry_where_end_time_is_before_start_time() {
        let context = Context::new().await;

        let original_start_time = utc(2026, 05, 09, 14, 10);
        let original_end_time = utc(2026, 05, 09, 14, 13);
        let original_time_entry = context
            .service
            .create_manual(original_start_time, original_end_time, context.task_ids[0])
            .await
            .unwrap();

        let modified_start_time = utc(2026, 05, 09, 14, 10);
        let modified_end_time = utc(2025, 05, 09, 15, 13);

        let modified_time_entry = context
            .service
            .edit_time_entry(
                original_time_entry.id,
                modified_start_time,
                modified_end_time,
            )
            .await;
        assert!(modified_time_entry.is_err());
        assert!(matches!(
            modified_time_entry.err().unwrap(),
            AppError::InvalidTimeRange
        ))
    }

    #[tokio::test]
    async fn assign_time_entry_to_new_task() {
        let context = Context::new().await;
        let start_time = utc(2026, 05, 09, 14, 10);
        let end_time = utc(2026, 05, 09, 14, 13);
        let time_entry = context
            .service
            .create_manual(start_time, end_time, context.task_ids[0])
            .await
            .unwrap();

        let result = context
            .service
            .assign_time_entry(time_entry.id, context.task_ids[2])
            .await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.start_time, time_entry.start_time);
        assert_eq!(result.end_time, time_entry.end_time);
        assert_eq!(result.task_id, context.task_ids[2]);
        assert_eq!(result.id, time_entry.id);
    }

    #[tokio::test]
    async fn assign_time_entry_to_new_task_with_invalid_entry_id() {
        let context = Context::new().await;

        let result = context
            .service
            .assign_time_entry(Uuid::new_v4(), context.task_ids[2])
            .await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::TimeEntryNotFound))
    }

    #[tokio::test]
    async fn assign_time_entry_to_new_task_with_invalid_task_id() {
        let context = Context::new().await;
        let start_time = utc(2026, 05, 09, 14, 10);
        let end_time = utc(2026, 05, 09, 14, 13);
        let time_entry = context
            .service
            .create_manual(start_time, end_time, context.task_ids[0])
            .await
            .unwrap();

        let result = context
            .service
            .assign_time_entry(time_entry.id, Uuid::new_v4())
            .await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::TaskNotFound))
    }
}
