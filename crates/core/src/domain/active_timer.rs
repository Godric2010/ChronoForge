use uuid::Uuid;
use chrono::{DateTime, Utc };

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveTimer {
    pub project_id: Uuid,
    pub task_id: Uuid,
    pub start_time: DateTime<Utc>,
}