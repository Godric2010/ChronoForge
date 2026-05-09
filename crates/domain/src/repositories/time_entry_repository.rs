use crate::types::TimeEntry;
use crate::errors::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TimeEntryRepository {
    async fn create(&self, time_entry: TimeEntry) -> AppResult<()>;
    async fn update(&self, time_entry: TimeEntry) -> AppResult<()>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TimeEntry>>;
    async fn find_by_task_id(&self, task_id: Uuid) -> AppResult<Vec<TimeEntry>>;
    async fn find_all(&self) -> AppResult<Vec<TimeEntry>>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
