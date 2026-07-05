use crate::types::Task;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: Task) -> anyhow::Result<()>;
    async fn upsert(&self, task: Task) -> anyhow::Result<()>;
    async fn update(&self, task: Task) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Task>>;
    async fn find_by_project_id(
        &self,
        project_id: Uuid,
        include_archived: bool,
    ) -> anyhow::Result<Vec<Task>>;
    async fn find_all(&self, include_archived: bool) -> anyhow::Result<Vec<Task>>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
