use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeEntry {
    pub id: Uuid,
    pub task_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
