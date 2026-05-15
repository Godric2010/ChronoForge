use async_trait::async_trait;
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
                INSERT INTO tasks (id, project_id, name)
                VALUES (?,?,?)
                "#,
        )
        .bind(task.id.to_string())
        .bind(task.project_id.to_string())
        .bind(&task.name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, task: Task) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                    UPDATE tasks
                    SET name = ?, project_id = ?
                    WHERE id = ?
                    "#,
        )
        .bind(&task.name)
        .bind(task.project_id.to_string())
        .bind(task.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Task>> {
        let row = sqlx::query_as::<_, TaskRow>(
            r#"
                    SELECT id, project_id, name
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
            id: Uuid::parse_str(&row.id).unwrap(),
            project_id: Uuid::parse_str(&row.project_id).unwrap(),
            name: row.name,
        }))
    }

    async fn find_by_project_id(&self, project_id: Uuid) -> anyhow::Result<Vec<Task>> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
                     SELECT id, project_id, name
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
            };
            tasks.push(task);
        }
        Ok(tasks)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Task>> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
                SELECT id, project_id, name
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
                    id: Uuid::parse_str(&row.id)?,
                    project_id: Uuid::parse_str(&row.project_id)?,
                    name: row.name,
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
}

