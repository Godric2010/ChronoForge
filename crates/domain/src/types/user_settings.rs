use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct UserSettings {
    pub monday_target_minutes: u32,
    pub tuesday_target_minutes: u32,
    pub wednesday_target_minutes: u32,
    pub thursday_target_minutes: u32,
    pub friday_target_minutes: u32,
    pub saturday_target_minutes: u32,
    pub sunday_target_minutes: u32,

    pub show_archived_projects: bool,
    pub show_archived_tasks: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
