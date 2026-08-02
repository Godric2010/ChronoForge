use crate::app_action::SetupAction;
use crate::app_render_helper::render_terminal_too_small_text;
use crate::event::{read_event, TuiEvent};
use crate::screens::setup::SetupScreen;
use crate::screens::welcome::WelcomeScreen;
use crate::screens::SetupScreens;
use anyhow::anyhow;
use crossterm::event::KeyEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use std::io::Stdout;
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub enum SetupResult {
    Quit,
    CreateNewDatabase(PathBuf),
    ConnectToDatabase(PathBuf),
}

#[derive(PartialEq)]
enum SetupState {
    Running,
    Done(SetupResult),
}
pub struct SetupApp {
    state: SetupState,
    active_screen: SetupScreens,
}

impl Default for SetupApp {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupApp {
    pub fn new() -> Self {
        Self {
            state: SetupState::Running,
            active_screen: SetupScreens::Welcome(WelcomeScreen::new("Chrono Forge".to_string())),
        }
    }

    pub async fn run(
        mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<SetupResult> {
        let mut tick_count: usize = 0;

        while self.state == SetupState::Running {
            let event = read_event(Duration::from_millis(250))?;
            match event {
                TuiEvent::Tick => {
                    match self.active_screen {
                        SetupScreens::Welcome(_) => {
                            tick_count += 1;
                            if tick_count > 4 {
                                self.active_screen = SetupScreens::Setup(SetupScreen::default());
                            }
                        }
                        SetupScreens::Setup(_) => {}
                    }

                    terminal.draw(|frame| {
                        self.render(frame);
                    })?;
                }
                TuiEvent::Input(ct_event) => {
                    terminal.draw(|frame| {
                        self.render(frame);
                    })?;
                    if let Some(key_event) = ct_event.as_key_event() {
                        if let Some(action) = self.handle_event(key_event) {
                            self.handle_setup_action(action);
                        }
                    }
                }
            }
        }

        match self.state {
            SetupState::Running => Err(anyhow!(
                "Setup is labeled as running, but the app is closing!"
            )),
            SetupState::Done(result) => Ok(result),
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if area.width < 150 || area.height < 40 {
            render_terminal_too_small_text(frame, area);
            return;
        }

        match &mut self.active_screen {
            SetupScreens::Welcome(welcome_screen) => welcome_screen.render(frame, area),
            SetupScreens::Setup(setup_screen) => setup_screen.render(frame, area),
        }
    }

    fn handle_event(&mut self, event: KeyEvent) -> Option<SetupAction> {
        match &mut self.active_screen {
            SetupScreens::Welcome(_) => None,
            SetupScreens::Setup(setup_screen) => setup_screen.handle_input(event),
        }
    }

    fn handle_setup_action(&mut self, action: SetupAction) {
        self.state = match action {
            SetupAction::Quit => SetupState::Done(SetupResult::Quit),
            SetupAction::CreateNewDatabase(path) => {
                SetupState::Done(SetupResult::CreateNewDatabase(path))
            }
            SetupAction::LinkNewDatabase(path) => {
                SetupState::Done(SetupResult::ConnectToDatabase(path))
            }
        }
    }
}
