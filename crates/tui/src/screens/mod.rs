use domain::types::Project;
use uuid::Uuid;
use crate::screens::project_overview::{ProjectOverviewViewModel, ProjectOverviewScreen};

mod project_overview;

pub enum ScreenType {
    ProjectOverview,
    Timer,
    Dashboard,
}

pub struct Screens {
    pub project_overview: ProjectOverviewScreen,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            project_overview: ProjectOverviewScreen::new(),
        }
    }
}

pub struct ScreenData {
    pub project_overview: ProjectOverviewViewModel,
}

impl ScreenData {
    pub fn new() -> Self {
        Self {
            project_overview: ProjectOverviewViewModel {
                projects: vec![
                    Project {
                        id: Uuid::new_v4(),
                        name: "Project One".to_string(),
                    },
                    Project {
                        id: Uuid::new_v4(),
                        name: "Project Two".to_string(),
                    },
                    Project {
                        id: Uuid::new_v4(),
                        name: "Project Three".to_string(),
                    },
                    Project {
                        id: Uuid::new_v4(),
                        name: "Project Four".to_string(),
                    },
                    Project {
                        id: Uuid::new_v4(),
                        name: "Project Five".to_string(),
                    },
                ],
            },
        }
    }
}
