use crate::screens::project_overview::ProjectOverviewScreen;
use crate::screens::welcome::WelcomeScreen;

pub mod project_overview;
pub mod welcome;

pub enum ScreenType {
    ProjectOverview,
    Timer,
    Dashboard,
}

pub struct Screens {
    pub welcome_screen: WelcomeScreen,
    pub project_overview: ProjectOverviewScreen,
    
}

impl Screens {
    pub fn new() -> Self {
        Self {
            welcome_screen: WelcomeScreen::new("Chrono Forge".to_string()),
            project_overview: ProjectOverviewScreen::new(),
        }
    }
}
