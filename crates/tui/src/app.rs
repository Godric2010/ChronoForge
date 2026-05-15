use crate::app_action::AppAction;
use crate::event::read_event;
use crate::screens::{Screen, Screens};
use crossterm::event::Event;
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use std::io::Stdout;

pub struct App {
    should_quit: bool,
    screens: Screens,
    current_screen: Screen,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: Screen::ProjectOverview,
            screens: Screens::new(),
        }
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| {
                self.render(frame);
            })?;

            let event = read_event()?;
            if let Some(action) = self.handle_event(event) {
                self.handle_action(action).await?
            }
        }

        Ok(())
    }
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        match self.current_screen {
            Screen::ProjectOverview => self.screens.project_overview.render(frame, area),
            Screen::Timer => {
                todo!()
            }
            Screen::Dashboard => {
                todo!()
            }
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<AppAction> {
        match self.current_screen {
            Screen::ProjectOverview => self.screens.project_overview.handle_event(event),
            Screen::Timer => {
                todo!()
            }
            Screen::Dashboard => {
                todo!()
            }
        }
    }

    async fn handle_action(&mut self, action: AppAction) -> anyhow::Result<()> {
        match action {
            AppAction::Quit => {
                self.should_quit = true;
            }
        }
        Ok(())
    }
}
