use domain::types::UserSettings;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct UserSettingsViewModel {
    pub db_path: PathBuf,
    pub user_settings: UserSettings,
}
