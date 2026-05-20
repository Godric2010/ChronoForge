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

    TimeEntrySelection,
    TimeEntryCreation,
    TimeEntryEdit,
    TimeEntryDeletion,
}
