use chrono::{DateTime, Utc};
use domain::types::{Project, Task, TimeEntry};

#[derive(Default, Clone)]
pub struct OverviewViewModel {
    pub projects: Vec<ProjectViewModel>,
}

#[derive(Clone)]
pub struct ProjectViewModel {
    pub project: Project,
    pub total_project_time_min: u32,
    pub time_limit: Option<u32>,
    pub tasks: Vec<TaskViewModel>,
}
#[derive(Clone)]
pub struct TaskViewModel {
    pub task: Task,
    pub total_task_time_min: u32,
    pub time_limit: Option<u32>,
    pub time_entries: Vec<TimeEntryViewModel>,
}

#[derive(Clone)]
pub struct TimeEntryViewModel {
    pub time_entry: TimeEntry,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
