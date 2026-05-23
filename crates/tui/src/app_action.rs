use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum AppAction {
    Quit,

    // Projects
    CreateProject(String),
    RenameProject(Uuid, String),
    DeleteProject(Uuid),

    // Tasks
    CreateTask(String, Uuid),
    RenameTask(Uuid, String),
    DeleteTask(Uuid),

    // Time Entries
    CreateTimeEntry(Uuid, DateTime<Utc>, DateTime<Utc>),
    EditTimeEntry(Uuid, DateTime<Utc>, DateTime<Utc>),
    DeleteTimeEntry(Uuid),

    // Timer
    StartTimer(Uuid),
    StopTimer,
}
