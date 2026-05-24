#[derive(PartialEq)]
pub enum Mode {
    ProjectSelection,
    ProjectCreation,
    ProjectEdit,
    ProjectDeletion,

    TaskSelection,
    TaskCreation,
    TaskEdit,
    TaskAssign,
    TaskDeletion,

    TimeEntrySelection,
    TimeEntryCreation,
    TimeEntryEdit,
    TimeEntryAssign,
    TimeEntryDeletion,
}
