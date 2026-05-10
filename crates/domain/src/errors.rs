use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Name cannot be empty")]
    EmptyName,

    #[error("Project not found")]
    ProjectNotFound,

    #[error("Task not found")]
    TaskNotFound,

    #[error("Time entry not found")]
    TimeEntryNotFound,

    #[error("A timer is already running")]
    TimerAlreadyRunning,

    #[error("No active timer")]
    NoActiveTimer,

    #[error("Invalid time range")]
    InvalidTimeRange,

    #[error("Storage error: {0}")]
    Storage(String),
}

pub type AppResult<T> = anyhow::Result<T, AppError>;
