use crate::domain::ActiveTimer;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ActiveTimerRepository {
    async fn create(&self, entry: ActiveTimer) -> AppResult<()>;
    async fn delete(&self) -> AppResult<()>;
    async fn get_active_timer(&self) -> AppResult<Option<ActiveTimer>>;
}