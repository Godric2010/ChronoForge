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
    
    // Timer
    StartTimer(Uuid),
    StopTimer,
}
