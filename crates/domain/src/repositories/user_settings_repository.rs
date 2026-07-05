use crate::types::UserSettings;
use async_trait::async_trait;

#[async_trait]
pub trait UserSettingsRepository: Send + Sync {
    async fn get(&self) -> anyhow::Result<UserSettings>;
    async fn update(&self, settings: UserSettings) -> anyhow::Result<()>;
}
