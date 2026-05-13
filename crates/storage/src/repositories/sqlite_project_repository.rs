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


