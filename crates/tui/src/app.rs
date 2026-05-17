use crate::app_action::AppAction;
use crate::event::read_event;
use crate::screens::{ScreenType, Screens};
use crate::TuiBackend;
use crossterm::event::Event;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::HorizontalAlignment::Center;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{symbols, Frame, Terminal};
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
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(" Chrono Forge ")
            .title_alignment(Center);
        frame.render_widget(main_block, area);

        let mut screen_rect = area;
        screen_rect.width -= 2;
        screen_rect.height -= 2;
        screen_rect.x += 1;
        screen_rect.y += 1;

        let app_layout_rects = Layout::vertical([
            Constraint::Length(3), // tab view
            Constraint::Length(1), // separator
            Constraint::Min(1),    // screen
            Constraint::Length(1), // separator
            Constraint::Length(3), // help bar
        ])
        .split(screen_rect);

        let separator =
            symbols::line::HORIZONTAL.repeat(app_layout_rects[1].width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = app_layout_rects[1];
        rect.x = rect.x + 1;
        frame.render_widget(separator_widget, rect);

        let mut help_text = String::new();
        let screen_area = app_layout_rects[2];
        match self.current_screen {
            ScreenType::ProjectOverview => {
                self.screens.project_overview.render(frame, screen_area);
                help_text = self.screens.project_overview.get_help_text();
            }
            ScreenType::Timer => {
                todo!()
            }
            ScreenType::Dashboard => {
                todo!()
            }
        }

        let separator =
            symbols::line::HORIZONTAL.repeat(app_layout_rects[3].width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = app_layout_rects[3];
        rect.x = rect.x + 1;
        frame.render_widget(separator_widget, rect);

        // help box
        let help_box = Paragraph::new(Line::from(help_text).alignment(Alignment::Center));
        let mut rect = app_layout_rects[4];
        rect.y += 1;
        frame.render_widget(help_box, rect);
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
