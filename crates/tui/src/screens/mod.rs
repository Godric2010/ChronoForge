use crate::screens::project_overview::ProjectOverviewScreen;

pub mod project_overview;

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

