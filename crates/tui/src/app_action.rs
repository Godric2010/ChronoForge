use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;

pub enum AppAction {
    Quit,

    // Projects
    CreateProject(String, Option<u32>),
    EditProject(Uuid, String, Option<u32>),
    DeleteProject(Uuid),
    ArchiveProject(Uuid),
    UnarchiveProject(Uuid),

    // Tasks
    CreateTask(String, Option<u32>, Uuid),
    RenameTask(Uuid, String, Option<u32>),
    AssignTask(Uuid, Uuid),
    DeleteTask(Uuid),
    ArchiveTask(Uuid),
    UnarchiveTask(Uuid),

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
    LinkNewDatabase(PathBuf),
    MoveDatabase(PathBuf),
    ToggleShowArchivedProjects(bool),
    ToggleShowArchivedTasks(bool),
    SetMondayWorkTarget(u32),
    SetTuesdayWorkTarget(u32),
    SetWednesdayWorkTarget(u32),
    SetThursdayWorkTarget(u32),
    SetFridayWorkTarget(u32),
    SetSaturdayWorkTarget(u32),
    SetSundayWorkTarget(u32),
}

pub enum SetupAction {
    Quit,
    CreateNewDatabase(PathBuf),
    LinkNewDatabase(PathBuf),
}
