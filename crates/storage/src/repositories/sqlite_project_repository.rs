use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::repositories::project_repository::ProjectRepository;
use domain::types::Project;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
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
    time_limit: u32,
    created_at: String,
    updated_at: String,
}

#[async_trait]
impl ProjectRepository for SQLiteProjectRepository {
    async fn create(&self, project: Project) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, time_limit, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(project.id.to_string())
        .bind(&project.name)
        .bind(project.time_limit)
        .bind(project.created_at.to_rfc3339())
        .bind(project.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert(&self, project: Project) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                INSERT INTO projects (id, name, time_limit, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?)
                    ON CONFLICT (id) DO UPDATE SET
                    name = excluded.name,
                    time_limit = excluded.time_limit,
                    updated_at = excluded.updated_at
                    WHERE excluded.updated_at > projects.updated_at
                    "#,
        )
        .bind(project.id.to_string())
        .bind(&project.name)
        .bind(project.time_limit)
        .bind(project.created_at.to_rfc3339())
        .bind(project.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, project: Project) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                    UPDATE projects
                    SET name = ?, time_limit = ?, updated_at = ?
                    WHERE id = ?
                  "#,
        )
        .bind(&project.name)
        .bind(project.time_limit)
        .bind(project.updated_at.to_rfc3339())
        .bind(project.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Project>> {
        let row = sqlx::query_as::<_, ProjectRow>(
            r#"
                SELECT id, name, time_limit, created_at, updated_at
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
            time_limit: row.time_limit,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc),
        }))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Project>> {
        let rows = sqlx::query_as::<_, ProjectRow>(
            r#"
            SELECT id, name, time_limit, created_at, updated_at
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
                    time_limit: row.time_limit,
                    created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc),
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
