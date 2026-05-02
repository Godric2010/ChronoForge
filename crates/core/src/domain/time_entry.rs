use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeEntry{
    pub id: Uuid,
    pub project_id: Uuid,
    pub task_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}