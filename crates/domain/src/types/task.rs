use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub time_limit: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
