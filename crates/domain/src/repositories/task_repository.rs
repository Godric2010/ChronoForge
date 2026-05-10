use crate::types::Task;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: Task) -> anyhow::Result<()>;
    async fn update(&self, task: Task) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Task>>;
    async fn find_by_project_id(&self, project_id: Uuid) -> anyhow::Result<Vec<Task>>;
    async fn fina_all(&self) -> anyhow::Result<Vec<Task>>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
