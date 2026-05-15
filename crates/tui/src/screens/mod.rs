use crate::screens::project_overview::ProjectOverviewScreen;

mod project_overview;

pub enum Screen{
    ProjectOverview,
    Timer,
    Dashboard,
}

pub struct Screens{
   pub project_overview: ProjectOverviewScreen,
}

impl Screens{
    pub fn new() -> Self{
        Self{
            project_overview: ProjectOverviewScreen::new(),
        }
    }
}