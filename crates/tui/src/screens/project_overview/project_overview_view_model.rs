use domain::types::{Project, Task};

#[derive(Default, Clone)]
pub struct ProjectOverviewViewModel {
    pub projects: Vec<ProjectViewModel>,
}

#[derive(Clone)]
pub struct ProjectViewModel {
    pub project: Project,
    pub total_project_time_min: u32,
    pub tasks: Vec<TaskViewModel>,
}
#[derive(Clone)]
pub struct TaskViewModel {
    pub task: Task,
    pub total_task_time_min: u32,
}

