use async_trait::async_trait;
use uuid::Uuid;

use crate::{types::Project, errors::AppResult};

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, project: Project) -> AppResult<()>;
    async fn update(&self, project: Project) -> AppResult<()>;
    async fn find_by_id(&self, id: &Uuid) -> AppResult<Project>;
    async fn find_all(&self) -> AppResult<Vec<Project>>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
