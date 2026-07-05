use crate::repositories::user_settings_repository::UserSettingsRepository;
use crate::types::UserSettings;

pub struct UserSettingsService<R>
where
    R: UserSettingsRepository,
{
    repository: R,
}

impl<R> UserSettingsService<R>
where
    R: UserSettingsRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_settings(&self) -> anyhow::Result<UserSettings> {
        self.repository.get().await
    }

    pub async fn set_show_archived_projects(&self, show: bool) -> anyhow::Result<()> {
        let mut settings = self.get_settings().await?;
        settings.show_archived_projects = show;
        settings.updated_at = chrono::Utc::now();
        self.repository.update(settings).await?;
        Ok(())
    }

    pub async fn set_show_archived_tasks(&self, show: bool) -> anyhow::Result<()> {
        let mut settings = self.get_settings().await?;
        settings.show_archived_tasks = show;
        settings.updated_at = chrono::Utc::now();
        self.repository.update(settings).await?;
        Ok(())
    }
}
