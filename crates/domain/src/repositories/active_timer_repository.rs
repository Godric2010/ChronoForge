use crate::types::ActiveTimer;
use async_trait::async_trait;

#[async_trait]
pub trait ActiveTimerRepository {
    async fn set(&self, entry: ActiveTimer) -> anyhow::Result<()>;
    async fn remove(&self) -> anyhow::Result<()>;
    async fn get_active_timer(&self) -> anyhow::Result<Option<ActiveTimer>>;
}
