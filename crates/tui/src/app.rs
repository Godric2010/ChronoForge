use crate::app_action::AppAction;
use crate::event::read_event;
use crate::screens::{ScreenType, Screens};
use crate::TuiBackend;
use crossterm::event::Event;
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use std::io::Stdout;

pub struct App {
    should_quit: bool,
    screens: Screens,
    current_screen: ScreenType,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: ScreenType::ProjectOverview,
            screens: Screens::new(),
        }
    }

    pub async fn run<B: TuiBackend>(
        &mut self,
        backend: &B,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<()> {
        while !self.should_quit {
            self.update_view_model(backend).await?;

            terminal.draw(|frame| {
                self.render(frame);
            })?;

            let event = read_event()?;
            if let Some(action) = self.handle_event(event) {
                self.handle_action(action, backend).await?
            }
        }

        Ok(())
    }

    async fn update_view_model<B: TuiBackend>(&mut self, backend: &B) -> anyhow::Result<()> {
        match self.current_screen {
            ScreenType::ProjectOverview => {
                let view_model = backend.load_projects().await?;
                self.screens
                    .project_overview
                    .set_view_model(view_model.clone());
                Ok(())
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

    async fn handle_action<B: TuiBackend>(
        &mut self,
        action: AppAction,
        backend: &B,
    ) -> anyhow::Result<()> {
        match action {
            AppAction::Quit => {
                self.should_quit = true;
            }
            AppAction::CreateProject(name) => {
                backend.create_project(&name).await?;
            }
            AppAction::RenameProject(id, name) => {
                backend.rename_project(id, &name).await?;
            }
            AppAction::DeleteProject(id) => {
                backend.delete_project(id).await?;
            }
        }
        Ok(())
    }
}
