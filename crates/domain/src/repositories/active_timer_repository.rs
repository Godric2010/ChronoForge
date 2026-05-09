use crate::types::ActiveTimer;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ActiveTimerRepository {
    async fn set(&self, entry: ActiveTimer) -> AppResult<()>;
    async fn remove(&self) -> AppResult<()>;
    async fn get_active_timer(&self) -> Option<ActiveTimer>;
}
