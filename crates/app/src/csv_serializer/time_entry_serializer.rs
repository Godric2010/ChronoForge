use std::path::Path;
use domain::services::time_entry_service::TimeEntryService;
use domain::types::TimeEntry;
use serde::Serialize;
use storage::repositories::sqlite_active_timer_repository::SqliteActiveTimerRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;
use storage::repositories::sqlite_time_entry_repository::SqliteTimeEntryRepository;

#[derive(Serialize)]
pub struct TimeEntryCsvRow {
    pub id: String,
    pub task_id: String,
    pub start_time: String,
    pub end_time: String,
}

impl From<TimeEntry> for TimeEntryCsvRow {
    fn from(value: TimeEntry) -> Self {
        Self {
            id: value.id.to_string(),
            task_id: value.task_id.to_string(),
            start_time: value.start_time.to_rfc3339(),
            end_time: value.end_time.to_rfc3339(),
        }
    }
}

pub struct TimeEntrySerializer<'a> {
    time_entry_service: &'a TimeEntryService<
        SQLiteTaskRepository,
        SqliteTimeEntryRepository,
        SqliteActiveTimerRepository,
    >,
}

impl<'a> TimeEntrySerializer<'a> {
    pub fn new(
        time_entry_service: &'a TimeEntryService<SQLiteTaskRepository, SqliteTimeEntryRepository, SqliteActiveTimerRepository>,
    ) -> Self {
        Self { time_entry_service }
    }
    
    pub async fn export_csv(&self, path: &Path) -> anyhow::Result<()> {

        let file_path = path.join("time_entries.csv");

        let mut writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        let time_entries = self.time_entry_service.find_all().await?;
        for time_entry in time_entries {
            let row = TimeEntryCsvRow::from(time_entry);
            writer.serialize(row)?;
        }
        writer.flush()?;
        Ok(())    }
}
