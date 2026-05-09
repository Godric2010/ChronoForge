use crate::types::Task;
use crate::errors::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: Task) -> AppResult<()>;
    async fn update(&self, task: Task) -> AppResult<()>;
    async fn find_by_id(&self, id: &Uuid) -> AppResult<Task>;
    async fn find_by_project_id(&self, project_id: Uuid) -> AppResult<Vec<Task>>;
    async fn fina_all(&self) -> AppResult<Vec<Task>>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
