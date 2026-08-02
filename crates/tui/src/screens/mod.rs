use crate::screens::overview::OverviewScreen;
use crate::screens::settings::SettingsScreen;
use crate::screens::setup::SetupScreen;
use crate::screens::welcome::WelcomeScreen;

pub(crate) mod dialog;
pub mod overview;
pub mod settings;
pub mod setup;
pub mod welcome;

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

impl Default for Screens {
    fn default() -> Self {
        Self::new()
    }
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

pub enum SetupScreens {
    Welcome(WelcomeScreen),
    Setup(SetupScreen),
}
