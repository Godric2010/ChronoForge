use crate::screens::overview::OverviewScreen;
use crate::screens::welcome::WelcomeScreen;

pub mod overview;
pub mod welcome;

pub enum ScreenType {
    ProjectOverview,
    Timer,
    Dashboard,
}

pub struct Screens {
    pub welcome_screen: WelcomeScreen,
    pub project_overview: OverviewScreen,
    
}

impl Screens {
    pub fn new() -> Self {
        Self {
            welcome_screen: WelcomeScreen::new("Chrono Forge".to_string()),
            project_overview: OverviewScreen::new(),
        }
    }
}
