use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveTimer {
    pub task_id: Uuid,
    pub start_time: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct DailyTimer {
    pub currently_active_timer_elapsed_minutes: Option<u32>,
    pub elapsed_time_in_minutes: u32,
    pub time_target_in_minutes: u32,
}
