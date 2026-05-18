#[derive(PartialEq)]
pub enum Mode {
    ProjectSelection,
    ProjectCreation,
    ProjectEdit,
    ProjectDeletion,
    
    TaskSelection,
    TaskCreation,
    TaskEdit,
    TaskDeletion,
    
    StartStopTimer,
}
