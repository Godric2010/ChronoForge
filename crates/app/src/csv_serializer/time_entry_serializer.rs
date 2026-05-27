use domain::services::time_entry_service::TimeEntryService;
use domain::types::TimeEntry;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::path::Path;
use storage::repositories::sqlite_active_timer_repository::SqliteActiveTimerRepository;
use storage::repositories::sqlite_task_repository::SQLiteTaskRepository;
use storage::repositories::sqlite_time_entry_repository::SqliteTimeEntryRepository;

#[derive(Serialize, Deserialize)]
pub struct TimeEntryCsvRow {
    pub id: String,
    pub task_id: String,
    pub start_time: String,
    pub end_time: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TimeEntry> for TimeEntryCsvRow {
    fn from(value: TimeEntry) -> Self {
        Self {
            id: value.id.to_string(),
            task_id: value.task_id.to_string(),
            start_time: value.start_time.to_rfc3339(),
            end_time: value.end_time.to_rfc3339(),
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

impl TryFrom<TimeEntryCsvRow> for TimeEntry {
    type Error = anyhow::Error;

    fn try_from(value: TimeEntryCsvRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            task_id: Uuid::parse_str(&value.task_id)?,
            start_time: DateTime::parse_from_rfc3339(&value.start_time)?.with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(&value.end_time)?.with_timezone(&Utc),
            created_at: DateTime::parse_from_rfc3339(&value.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&value.updated_at)?.with_timezone(&Utc),
        })
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
        time_entry_service: &'a TimeEntryService<
            SQLiteTaskRepository,
            SqliteTimeEntryRepository,
            SqliteActiveTimerRepository,
        >,
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
        Ok(())
    }

    pub async fn import_csv(&self, path: &Path) -> anyhow::Result<()> {
        let file_path = path.join("time_entries.csv");
        if !file_path.exists() {
            anyhow::bail!("Missing time entry.csv file at {}", path.display());
        }

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;

        for result in reader.deserialize() {
            let row: TimeEntryCsvRow = result?;
            let time_entry = TimeEntry::try_from(row)?;
            self.time_entry_service.upsert(time_entry).await?;
        }

        Ok(())
    }
}
