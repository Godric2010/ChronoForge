use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::repositories::time_entry_repository::TimeEntryRepository;
use domain::types::TimeEntry;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteTimeEntryRepository {
    pool: SqlitePool,
}

impl SqliteTimeEntryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    fn build_time_entry(&self, row: &TimeEntriesRow) -> anyhow::Result<TimeEntry> {
        Ok(TimeEntry {
            id: Uuid::parse_str(&row.id)?,
            task_id: Uuid::parse_str(&row.task_id)?,
            start_time: DateTime::parse_from_rfc3339(&row.start_time)?.with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(&row.end_time)?.with_timezone(&Utc),
        })
    }
}

#[async_trait]
impl TimeEntryRepository for SqliteTimeEntryRepository {
    async fn create(&self, time_entry: TimeEntry) -> anyhow::Result<()> {
        sqlx::query(
            r#"
INSERT INTO time_entries (id, task_id, start_time, end_time)
VALUES (?, ?, ?, ?)
"#,
        )
        .bind(time_entry.id.to_string())
        .bind(time_entry.task_id.to_string())
        .bind(time_entry.start_time.to_rfc3339())
        .bind(time_entry.end_time.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, time_entry: TimeEntry) -> anyhow::Result<()> {
        sqlx::query(
            r#"
UPDATE time_entries
SET task_id = ?, start_time = ?, end_time = ?
WHERE id = ?
"#,
        )
        .bind(time_entry.task_id.to_string())
        .bind(time_entry.start_time.to_rfc3339())
        .bind(time_entry.end_time.to_rfc3339())
        .bind(time_entry.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<TimeEntry>> {
        let row = sqlx::query_as::<_, TimeEntriesRow>(
            r#"
SELECT id, task_id, start_time, end_time
FROM time_entries
WHERE id = ?
"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let time_entry = self.build_time_entry(&row)?;
        Ok(Some(time_entry))
    }

    async fn find_by_task_id(&self, task_id: Uuid) -> anyhow::Result<Vec<TimeEntry>> {
        let rows = sqlx::query_as::<_, TimeEntriesRow>(
            r#"
SELECT id, task_id, start_time, end_time
FROM time_entries
WHERE task_id = ?
"#,
        )
        .bind(task_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut time_entries = Vec::new();
        for row in rows {
            time_entries.push(self.build_time_entry(&row)?);
        }

        Ok(time_entries)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<TimeEntry>> {
        let rows = sqlx::query_as::<_, TimeEntriesRow>(
            r#"
SELECT id, task_id, start_time, end_time
FROM time_entries
"#,
        )
        .fetch_all(&self.pool)
        .await?;

        let time_entries = rows
            .into_iter()
            .map(|row| {
                let entry = self.build_time_entry(&row)?;
                Ok(entry)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(time_entries)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(r#"DELETE FROM time_entries WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct TimeEntriesRow {
    id: String,
    task_id: String,
    start_time: String,
    end_time: String,
}
