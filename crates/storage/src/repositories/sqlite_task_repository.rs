use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::repositories::task_repository::TaskRepository;
use domain::types::Task;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SQLiteTaskRepository {
    pool: SqlitePool,
}

impl SQLiteTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for SQLiteTaskRepository {
    async fn create(&self, task: Task) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                INSERT INTO tasks (id, project_id, name, time_limit, is_archived, created_at, updated_at)
                VALUES (?,?,?,?,?,?,?)
                "#,
        )
        .bind(task.id.to_string())
        .bind(task.project_id.to_string())
        .bind(&task.name)
        .bind(task.time_limit)
        .bind(if task.is_archived {1} else {0})
        .bind(task.created_at.to_rfc3339())
        .bind(task.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert(&self, task: Task) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                INSERT INTO tasks (id, project_id, name, time_limit, is_archived, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT (id) DO UPDATE SET
                    name = excluded.name,
                    project_id = excluded.project_id,
                    is_archived = excluded.is_archived,
                    time_limit = excluded.time_limit,
                    updated_at = excluded.updated_at
                    WHERE excluded.updated_at > tasks.updated_at
                    "#,
        )
        .bind(task.id.to_string())
        .bind(task.project_id.to_string())
        .bind(&task.name)
        .bind(task.time_limit)
        .bind(if task.is_archived {1} else {0})
        .bind(task.created_at.to_rfc3339())
        .bind(task.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, task: Task) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                    UPDATE tasks
                    SET name = ?, project_id = ?, time_limit = ?, is_archived = ?, updated_at = ?, created_at = ?
                    WHERE id = ?
                    "#,
        )
        .bind(&task.name)
        .bind(task.project_id.to_string())
        .bind(task.time_limit)
        .bind(if task.is_archived {1} else {0})
        .bind(task.created_at.to_rfc3339())
        .bind(task.updated_at.to_rfc3339())
        .bind(task.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Task>> {
        let row = sqlx::query_as::<_, TaskRow>(
            r#"
                    SELECT id, project_id, name, time_limit, is_archived, created_at, updated_at
                    FROM tasks
                    WHERE id = ?
                    "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(row.parse()?))
    }

    async fn find_by_project_id(
        &self,
        project_id: Uuid,
        include_archived: bool,
    ) -> anyhow::Result<Vec<Task>> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
                     SELECT id, project_id, name, time_limit, is_archived, created_at, updated_at
                     FROM tasks
                     WHERE project_id = ?1 AND (?2 = 1 OR is_archived = 0)
                     ORDER BY name
                     "#,
        )
        .bind(project_id.to_string())
        .bind(if include_archived { 1 } else { 0 })
        .fetch_all(&self.pool)
        .await?;

        let tasks = rows
            .iter()
            .map(|row| row.parse())
            .collect::<anyhow::Result<Vec<Task>>>()?;
        Ok(tasks)
    }

    async fn find_all(&self, include_archived: bool) -> anyhow::Result<Vec<Task>> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
                SELECT id, project_id, name, time_limit, is_archived, created_at, updated_at
                FROM tasks
                WHERE (?1 = 1 OR is_archived = 0)
                ORDER BY name
                "#,
        )
        .bind(if include_archived { 1 } else { 0 })
        .fetch_all(&self.pool)
        .await?;

        let tasks = rows
            .into_iter()
            .map(|row| row.parse())
            .collect::<anyhow::Result<Vec<Task>>>()?;
        Ok(tasks)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                  DELETE FROM tasks
                  WHERE id = ?
                  "#,
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct TaskRow {
    id: String,
    project_id: String,
    name: String,
    time_limit: u32,
    is_archived: i32,
    created_at: String,
    updated_at: String,
}

impl TaskRow {
    pub fn parse(&self) -> anyhow::Result<Task> {
        Ok(Task {
            id: Uuid::parse_str(&self.id)?,
            project_id: Uuid::parse_str(&self.project_id)?,
            name: self.name.clone(),
            time_limit: if self.time_limit > 0 {
                Some(self.time_limit)
            } else {
                None
            },
            is_archived: self.is_archived != 0,
            created_at: DateTime::parse_from_rfc3339(&self.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at)?.with_timezone(&Utc),
        })
    }
}
