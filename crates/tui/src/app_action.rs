use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum AppAction {
    Quit,

    // Projects
    CreateProject(String, Option<u32>),
    EditProject(Uuid, String, Option<u32>),
    DeleteProject(Uuid),

    // Tasks
    CreateTask(String, Uuid),
    RenameTask(Uuid, String),
    AssignTask(Uuid, Uuid),
    DeleteTask(Uuid),

    // Time Entries
    CreateTimeEntry(Uuid, DateTime<Utc>, DateTime<Utc>),
    EditTimeEntry(Uuid, DateTime<Utc>, DateTime<Utc>),
    AssignTimeEntry(Uuid, Uuid),
    DeleteTimeEntry(Uuid),

    // Timer
    StartTimer(Uuid),
    StopTimer,

    // Settings
    ImportCsv(String),
    ExportCsv(String),
}
