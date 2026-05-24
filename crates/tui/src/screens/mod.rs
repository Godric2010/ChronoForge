use crate::screens::overview::OverviewScreen;
use crate::screens::settings::SettingsScreen;
use crate::screens::welcome::WelcomeScreen;

pub mod overview;
pub mod welcome;
pub mod settings;

#[derive(Copy, Clone)]
pub enum ScreenType {
    Overview,
    Settings,
}

pub struct Screens {
    pub welcome_screen: WelcomeScreen,
    pub overview: OverviewScreen,
    pub settings: SettingsScreen,
    
}

impl Screens {
    pub fn new() -> Self {
        Self {
            welcome_screen: WelcomeScreen::new("Chrono Forge".to_string()),
            overview: OverviewScreen::new(),
            settings: SettingsScreen::new(),
        }
    }
}
