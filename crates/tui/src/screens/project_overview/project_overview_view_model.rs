use domain::types::Project;

#[derive(Default, Clone)]
pub struct ProjectOverviewViewModel {
    pub projects: Vec<Project>,
}
