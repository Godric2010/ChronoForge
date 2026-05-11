use async_trait::async_trait;
use domain::repositories::task_repository::TaskRepository;
use domain::types::Task;
use sqlx::SqlitePool;
use uuid::Uuid;

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

#[cfg(test)]
mod task_repository_tests {
    use super::*;
    use sqlx::{Pool, Sqlite};

    async fn setup() -> Pool<Sqlite> {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
        pool
    }
    async fn create_test_project(pool: &SqlitePool, id: Uuid, name: &str) {
        sqlx::query(
            r#"
        INSERT INTO projects (id, name)
        VALUES (?, ?)
        "#,
        )
        .bind(id.to_string())
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
    }
    #[tokio::test]
    async fn create_project_and_store_it() {
        let pool = setup().await;
        let repository = SQLiteTaskRepository::new(pool.clone());
        let project_id = Uuid::new_v4();
        create_test_project(&pool, project_id, "Project1").await;

        let task = Task {
            id: Uuid::new_v4(),
            project_id: project_id.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task.clone()).await.unwrap();

        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());
        let stored_task = stored_task.unwrap();
        assert_eq!(stored_task.id, task.id);
        assert_eq!(stored_task.project_id, task.project_id);
        assert_eq!(stored_task.name, task.name);
    }

    #[tokio::test]
    async fn update_project_and_store_it() {
        let pool = setup().await;
        let repository = SQLiteTaskRepository::new(pool.clone());
        let project_id = Uuid::new_v4();
        let project_alt_id = Uuid::new_v4();
        create_test_project(&pool, project_id, "Project1").await;
        create_test_project(&pool, project_alt_id, "Project2").await;

        let task = Task {
            id: Uuid::new_v4(),
            project_id: project_id.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task.clone()).await.unwrap();

        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());

        let updated_name_task = Task {
            id: task.id,
            project_id: task.project_id,
            name: "TaskyMcTask".to_string(),
        };

        repository.update(updated_name_task.clone()).await.unwrap();
        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());
        let stored_task = stored_task.unwrap();
        assert_eq!(stored_task.id, updated_name_task.id);
        assert_eq!(stored_task.project_id, updated_name_task.project_id);
        assert_eq!(stored_task.name, updated_name_task.name);

        let updated_project_task = Task {
            id: updated_name_task.id,
            project_id: project_alt_id,
            name: updated_name_task.name,
        };
        repository
            .update(updated_project_task.clone())
            .await
            .unwrap();
        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());
        let stored_task = stored_task.unwrap();
        assert_eq!(stored_task.id, updated_project_task.id);
        assert_eq!(stored_task.project_id, updated_project_task.project_id);
        assert_eq!(stored_task.name, updated_project_task.name);
    }

    #[tokio::test]
    async fn find_tasks_by_project_id() {
        let pool = setup().await;
        let repository = SQLiteTaskRepository::new(pool.clone());
        let project_a = Uuid::new_v4();
        let project_b = Uuid::new_v4();
        create_test_project(&pool, project_a, "Project1").await;
        create_test_project(&pool, project_b, "Project2").await;

        let task_a = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask02".to_string(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_c.clone()).await.unwrap();

        let result = repository.find_by_project_id(project_a).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, task_a.id);
        assert_eq!(result[0].project_id, project_a);
        assert_eq!(result[0].name, task_a.name);
        assert_eq!(result[1].id, task_b.id);
        assert_eq!(result[1].project_id, project_a);
        assert_eq!(result[1].name, task_b.name);
    }
    #[tokio::test]
    async fn find_all_tasks() {
        let pool = setup().await;
        let repository = SQLiteTaskRepository::new(pool.clone());
        let project_a = Uuid::new_v4();
        let project_b = Uuid::new_v4();
        create_test_project(&pool, project_a, "Project1").await;
        create_test_project(&pool, project_b, "Project2").await;

        let task_a = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask02".to_string(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_c.clone()).await.unwrap();

        let result = repository.find_all().await.unwrap();
        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn delete_task_expect_it_to_be_gone() {
        let pool = setup().await;
        let repository = SQLiteTaskRepository::new(pool.clone());
        let project_a = Uuid::new_v4();
        let project_b = Uuid::new_v4();
        create_test_project(&pool, project_a, "Project1").await;
        create_test_project(&pool, project_b, "Project2").await;

        let task_a = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask02".to_string(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_c.clone()).await.unwrap();

        let result = repository.delete(task_b.id).await;
        assert!(result.is_ok());

        let result = repository.find_by_id(&task_b.id).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_none());
    }
}
