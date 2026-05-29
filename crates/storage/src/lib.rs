use crate::repositories::sqlite_project_repository::SQLiteProjectRepository;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod integration_tests;
pub mod repositories;

#[allow(dead_code)]
pub struct StorageManager {
    pool: SqlitePool,
    project_repo: SQLiteProjectRepository,
}

impl StorageManager {
    pub async fn new(db_url: &str) -> anyhow::Result<StorageManager> {
        let pool = SqlitePool::connect(db_url).await?;
        sqlx::migrate!("../../migrations").run(&pool).await?;

        let project_repo = SQLiteProjectRepository::new(pool.clone());
        Ok(StorageManager { pool, project_repo })
    }

    pub fn project_repository(&self) -> &SQLiteProjectRepository {
        &self.project_repo
    }
}
