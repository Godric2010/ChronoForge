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
    Help,
}

#[derive(Clone, Copy)]
pub(crate) enum ProjectsModeActions {
    New,
    Edit,
    Delete,
}

#[derive(Clone, Copy)]
pub(crate) enum TasksModeActions {
    New,
    Edit,
    AssignToProject,
    Delete,
}

#[derive(Clone, Copy)]
pub(crate) enum TimeEntriesModeActions {
    New,
    Edit,
    AssignToTask,
    Delete,
}
