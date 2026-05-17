use uuid::Uuid;

pub enum AppAction {
    Quit,

    // Projects
    CreateProject(String),
    RenameProject(Uuid, String),
    DeleteProject(Uuid),
}
