use crate::errors::{AppError, AppResult};
use crate::repositories::active_timer_repository::ActiveTimerRepository;
use crate::repositories::project_repository::ProjectRepository;
use crate::repositories::task_repository::TaskRepository;
use crate::repositories::time_entry_repository::TimeEntryRepository;
use crate::repositories::user_settings_repository::UserSettingsRepository;
use crate::types::DailyTimer;
use chrono::{DateTime, Datelike, Local, Utc, Weekday};

#[allow(dead_code)]
pub struct ReportService<
    P: ProjectRepository,
    T: TaskRepository,
    E: TimeEntryRepository,
    A: ActiveTimerRepository,
    S: UserSettingsRepository,
> {
    project_repository: P,
    task_repository: T,
    time_entry_repository: E,
    active_timer_repository: A,
    settings_repository: S,
}

impl<
        P: ProjectRepository,
        T: TaskRepository,
        E: TimeEntryRepository,
        A: ActiveTimerRepository,
        S: UserSettingsRepository,
    > ReportService<P, T, E, A, S>
{
    pub fn new(
        project_repository: P,
        task_repository: T,
        time_entry_repository: E,
        active_timer_repository: A,
        settings_repository: S,
    ) -> Self {
        Self {
            project_repository,
            task_repository,
            time_entry_repository,
            active_timer_repository,
            settings_repository,
        }
    }

    pub async fn get_daily_work_time(&self) -> AppResult<DailyTimer> {
        let today = Utc::now().with_timezone(&Local);
        let elapsed_time = self.get_work_time_of_day(&today).await?;
        let work_target_time = self.get_work_target_of_day(&today).await?;
        let active_time = self
            .get_currently_active_timer_elapsed_minutes(&today)
            .await?;
        Ok(DailyTimer {
            currently_active_timer_elapsed_minutes: active_time,
            elapsed_time_in_minutes: elapsed_time,
            time_target_in_minutes: work_target_time,
        })
    }

    async fn get_work_target_of_day(&self, today: &DateTime<Local>) -> AppResult<u32> {
        let settings = self.settings_repository.get().await;
        if settings.is_err() {
            return Err(AppError::Storage(
                "Could not fetch settings from database".to_string(),
            ));
        }
        let settings = settings.unwrap();

        let work_target = match today.weekday() {
            Weekday::Mon => settings.monday_target_minutes,
            Weekday::Tue => settings.tuesday_target_minutes,
            Weekday::Wed => settings.wednesday_target_minutes,
            Weekday::Thu => settings.thursday_target_minutes,
            Weekday::Fri => settings.friday_target_minutes,
            Weekday::Sat => settings.saturday_target_minutes,
            Weekday::Sun => settings.sunday_target_minutes,
        };
        Ok(work_target)
    }

    async fn get_currently_active_timer_elapsed_minutes(
        &self,
        today: &DateTime<Local>,
    ) -> AppResult<Option<u32>> {
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
        let start_time = active_timer.start_time.with_timezone(&Local);

        let elapsed_minutes = (*today - start_time).num_minutes() as u32;
        Ok(Some(elapsed_minutes))
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

    async fn get_work_time_of_day(&self, today: &DateTime<Local>) -> AppResult<u32> {
        let all_time_entries = self.time_entry_repository.find_all().await;
        if all_time_entries.is_err() {
            return Err(AppError::Storage(format!(
                "Error when fecthing all time entries: {}",
                all_time_entries.err().unwrap()
            )));
        }

        let all_time_entries = all_time_entries.unwrap();
        let mut time_worked_today: u32 = 0;
        all_time_entries.iter().for_each(|entry| {
            let entry_end_time = entry.end_time.with_timezone(&Local);
            if today.day() == entry_end_time.day()
                && today.month() == entry_end_time.month()
                && today.year() == entry_end_time.year()
            {
                let entry_elapsed = entry_end_time - entry.start_time.with_timezone(&Local);
                let elapsed_minutes = entry_elapsed.num_minutes() as u32;
                time_worked_today += elapsed_minutes;
            }
        });
        Ok(time_worked_today)
    }
}
