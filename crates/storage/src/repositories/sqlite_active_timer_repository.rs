use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::repositories::active_timer_repository::ActiveTimerRepository;
use domain::types::ActiveTimer;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteActiveTimerRepository {
    pool: SqlitePool,
}

impl SqliteActiveTimerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ActiveTimerRepository for SqliteActiveTimerRepository {
    async fn set(&self, entry: ActiveTimer) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                    INSERT INTO active_timer (id, task_id, start_time)
                    VALUES (1, ?, ?)
                    "#,
        )
        .bind(entry.task_id.to_string())
        .bind(entry.start_time.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove(&self) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                     DELETE FROM active_timer
                     WHERE id = 1
                    "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_active_timer(&self) -> anyhow::Result<Option<ActiveTimer>> {
        let row = sqlx::query_as::<_, ActiveTimerRow>(
            r#"
                    SELECT id, task_id, start_time
                    FROM active_timer
                    WHERE id = 1
                    "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(ActiveTimer {
            task_id: Uuid::parse_str(&row.task_id).unwrap(),
            start_time: DateTime::parse_from_rfc3339(&row.start_time)?.with_timezone(&Utc),
        }))
    }
}

#[derive(sqlx::FromRow)]
struct ActiveTimerRow {
    id: i32,
    task_id: String,
    start_time: String,
}

