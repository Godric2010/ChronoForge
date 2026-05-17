use crate::app_action::AppAction;
use crate::event::read_event;
use crate::screens::{ScreenData, ScreenType, Screens};
use crossterm::event::Event;
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use std::io::Stdout;

pub struct App {
    should_quit: bool,
    screens: Screens,
    current_screen: ScreenType,
    app_data: ScreenData,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: ScreenType::ProjectOverview,
            screens: Screens::new(),
            app_data: ScreenData::new(),
        }
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<()> {
        while !self.should_quit {
            self.update_view_model();

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

    fn update_view_model(&mut self) {
        match self.current_screen {
            ScreenType::ProjectOverview => {
                self.screens
                    .project_overview
                    .set_view_model(self.app_data.project_overview.clone());
            }
            ScreenType::Timer => {
                todo!()
            }
            ScreenType::Dashboard => {
                todo!()
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        match self.current_screen {
            ScreenType::ProjectOverview => self.screens.project_overview.render(frame, area),
            ScreenType::Timer => {
                todo!()
            }
            ScreenType::Dashboard => {
                todo!()
            }
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<AppAction> {
        match self.current_screen {
            ScreenType::ProjectOverview => self.screens.project_overview.handle_event(event),
            ScreenType::Timer => {
                todo!()
            }
            ScreenType::Dashboard => {
                todo!()
            }
        }
    }

    async fn handle_action(&mut self, action: AppAction) -> anyhow::Result<()> {
        match action {
            AppAction::Quit => {
                self.should_quit = true;
            }
            AppAction::CreateProject(_) => {}
            AppAction::RenameProject(_, _) => {}
            AppAction::DeleteProject(_) => {}
        }
        Ok(())
    }
}
