use async_trait::async_trait;
use domain::repositories::project_repository::ProjectRepository;
use domain::types::Project;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SQLiteProjectRepository {
    pool: SqlitePool,
}

impl SQLiteProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ProjectRow {
    id: String,
    name: String,
}

#[async_trait]
impl ProjectRepository for SQLiteProjectRepository {
    async fn create(&self, project: Project) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, name)
            VALUES (?, ?)
            "#,
        )
        .bind(project.id.to_string())
        .bind(&project.name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, project: Project) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                    UPDATE projects
                    SET name = ?
                    WHERE id = ?
                  "#,
        )
        .bind(&project.name)
        .bind(&project.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Project>> {
        let row = sqlx::query_as::<_, ProjectRow>(
            r#"
                SELECT id, name
                FROM projects
                WHERE id = ?
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(Project {
            id: Uuid::parse_str(&row.id)?,
            name: row.name,
        }))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Project>> {
        let rows = sqlx::query_as::<_, ProjectRow>(
            r#"
            SELECT id, name
            FROM projects
            ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let projects = rows
            .into_iter()
            .map(|row| {
                Ok(Project {
                    id: Uuid::parse_str(&row.id)?,
                    name: row.name,
                })
            })
            .collect::<anyhow::Result<Vec<Project>>>()?;
        Ok(projects)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                    DELETE FROM projects
                    WHERE id = ?
                   "#,
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod project_repository_tests {
    use super::*;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn create_test_repository() -> SQLiteProjectRepository {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("../../migrations").run(&pool).await.unwrap();

        SQLiteProjectRepository::new(pool)
    }

    #[tokio::test]
    async fn create_project_should_store_it() {
        let repository = create_test_repository().await;

        let project = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
        };

        repository.create(project.clone()).await.unwrap();

        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(project.id, stored_project.id);
        assert_eq!(project.name, stored_project.name);
    }

    #[tokio::test]
    async fn find_by_id_should_return_none_when_project_doesnt_exist() {
        let repository = create_test_repository().await;

        let project = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
        };
        repository.create(project.clone()).await.unwrap();

        let invalid_project_id = Uuid::new_v4();
        let stored_project = repository.find_by_id(&invalid_project_id).await.unwrap();
        assert!(stored_project.is_none());
    }

    #[tokio::test]
    async fn find_all_should_return_all_projects() {
        let repository = create_test_repository().await;
        let project_a = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
        };

        let project_b = Project {
            id: Uuid::new_v4(),
            name: "MazeGame".to_string(),
        };

        let project_c = Project {
            id: Uuid::new_v4(),
            name: "JAREP".to_string(),
        };
        repository.create(project_a.clone()).await.unwrap();
        repository.create(project_b.clone()).await.unwrap();
        repository.create(project_c.clone()).await.unwrap();

        let stored_project = repository.find_all().await.unwrap();
        assert_eq!(stored_project.len(), 3);
        // The order here is by design. The database is supposed to sort the entries by name.
        // Therefore, projects need to be assigned manually to their respected original.
        assert_eq!(stored_project[0].id, project_a.id);
        assert_eq!(stored_project[0].name, project_a.name);
        assert_eq!(stored_project[2].id, project_b.id);
        assert_eq!(stored_project[2].name, project_b.name);
        assert_eq!(stored_project[1].id, project_c.id);
        assert_eq!(stored_project[1].name, project_c.name);
    }

    #[tokio::test]
    async fn find_all_should_return_empty_list_when_no_projects_exist() {
        let repository = create_test_repository().await;
        let stored_project = repository.find_all().await.unwrap();
        assert_eq!(stored_project.len(), 0);
    }

    #[tokio::test]
    async fn update_should_update_existing_project() {
        let repository = create_test_repository().await;
        let project_original = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
        };

        repository.create(project_original.clone()).await.unwrap();

        let updated_project = Project {
            id: project_original.id,
            name: "ZeitSchmiede".to_string(),
        };
        repository.update(updated_project.clone()).await.unwrap();
        let stored_project = repository.find_by_id(&project_original.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(updated_project.id, stored_project.id);
        assert_eq!(updated_project.name, stored_project.name);
    }

    #[tokio::test]
    async fn delete_should_delete_existing_project() {
        let repository = create_test_repository().await;
        let project = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
        };

        repository.create(project.clone()).await.unwrap();
        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_some());

        repository.delete(project.id).await.unwrap();
        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_none());
    }
}
