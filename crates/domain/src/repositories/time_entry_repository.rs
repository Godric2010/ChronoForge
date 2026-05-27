use crate::types::TimeEntry;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TimeEntryRepository {
    async fn create(&self, time_entry: TimeEntry) -> anyhow::Result<()>;
    async fn upsert(&self, time_entry: TimeEntry) -> anyhow::Result<()>;
    async fn update(&self, time_entry: TimeEntry) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<TimeEntry>>;
    async fn find_by_task_id(&self, task_id: Uuid) -> anyhow::Result<Vec<TimeEntry>>;
    async fn find_all(&self) -> anyhow::Result<Vec<TimeEntry>>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
