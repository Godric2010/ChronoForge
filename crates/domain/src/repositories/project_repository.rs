use async_trait::async_trait;
use uuid::Uuid;

use crate::types::Project;

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, project: Project) -> anyhow::Result<()>;
    async fn update(&self, project: Project) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Project>>;
    async fn find_all(&self) -> anyhow::Result<Vec<Project>>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
