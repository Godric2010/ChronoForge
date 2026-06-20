#[derive(PartialEq)]
pub enum Mode {
    Projects,
    Tasks,
    TimeEntries,
}

#[derive(Copy, Clone)]
pub(crate) enum OverviewGeneralActions {
    Quit,
    NextMode,
    PrevMode,
    ToggleTimer,
}

#[derive(Clone, Copy)]
pub(crate) enum ProjectsModeActions {
    NewProject,
    EditProject,
    DeleteProject,
}

#[derive(Clone, Copy)]
pub(crate) enum TasksModeActions {
    NewTask,
    EditTask,
    AssignTask,
    DeleteTask,
}

#[derive(Clone, Copy)]
pub(crate) enum TimeEntriesModeActions {
    NewTimeEntry,
    EditTimeEntry,
    AssignTimeEntry,
    DeleteTimeEntry,
}
