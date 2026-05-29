use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveTimer {
    pub task_id: Uuid,
    pub start_time: DateTime<Utc>,
}
