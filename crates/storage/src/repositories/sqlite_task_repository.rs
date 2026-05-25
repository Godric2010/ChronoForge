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
                INSERT INTO tasks (id, project_id, name, time_limit, created_at, updated_at)
                VALUES (?,?,?,?,?,?)
                "#,
        )
        .bind(task.id.to_string())
        .bind(task.project_id.to_string())
        .bind(&task.name)
        .bind(task.time_limit)
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
                    SET name = ?, project_id = ?, time_limit = ?, updated_at = ?, created_at = ?
                    WHERE id = ?
                    "#,
        )
        .bind(&task.name)
        .bind(task.project_id.to_string())
        .bind(task.time_limit)
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
                    SELECT id, project_id, name, time_limit, created_at, updated_at
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

        Ok(Some(Task {
            id: Uuid::parse_str(&row.id)?,
            project_id: Uuid::parse_str(&row.project_id)?,
            name: row.name,
            time_limit: row.time_limit,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc),
        }))
    }

    async fn find_by_project_id(&self, project_id: Uuid) -> anyhow::Result<Vec<Task>> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
                     SELECT id, project_id, name, time_limit, created_at, updated_at
                     FROM tasks
                     WHERE project_id = ?
                     ORDER BY name
                     "#,
        )
        .bind(project_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut tasks = Vec::<Task>::new();
        for row in rows {
            let task = Task {
                id: Uuid::parse_str(&row.id).unwrap(),
                project_id: Uuid::parse_str(&row.project_id).unwrap(),
                name: row.name,
                time_limit: row.time_limit,
                created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc),
            };
            tasks.push(task);
        }
        Ok(tasks)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Task>> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
                SELECT id, project_id, name, time_limit, created_at, updated_at
                FROM tasks
                ORDER BY name
                "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let tasks = rows
            .into_iter()
            .map(|row| {
                Ok(Task {
                    id: Uuid::parse_str(&row.id).unwrap(),
                    project_id: Uuid::parse_str(&row.project_id).unwrap(),
                    name: row.name,
                    time_limit: row.time_limit,
                    created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc),
                })
            })
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
    created_at: String,
    updated_at: String,
}
