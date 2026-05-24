use crate::app_action::AppAction;
use crate::event::{read_event, TuiEvent};
use crate::screens::{ScreenType, Screens};
use crate::widgets::active_timer::ActiveTimer;
use crate::TuiBackend;
use crossterm::event::Event;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{symbols, Frame, Terminal};
use std::io::Stdout;
use std::time::Duration;

pub struct App {
    should_quit: bool,
    welcome_active: bool,
    screens: Screens,
    current_screen: ScreenType,
    active_timer: ActiveTimer,
    enforce_vm_update_on_next_tick: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            welcome_active: true,
            current_screen: ScreenType::ProjectOverview,
            screens: Screens::new(),
            active_timer: ActiveTimer::new(),
            enforce_vm_update_on_next_tick: false,
        }
    }

    pub async fn run<B: TuiBackend>(
        &mut self,
        backend: &B,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<()> {
        self.update_view_model(backend).await?;
        self.update_tick(backend).await?;

        let mut tick_count: usize = 0;

        while !self.should_quit {
            let event = read_event(Duration::from_millis(250))?;
            match event {
                TuiEvent::Tick => {
                    if self.welcome_active {
                        tick_count += 1;

                        if tick_count > 4 {
                            self.welcome_active = false;
                        }
                    }

                    self.update_tick(backend).await?;
                    terminal.draw(|frame| {
                        self.render(frame);
                    })?;
                }
                TuiEvent::Input(ct_event) => {
                    self.update_view_model(backend).await?;

                    terminal.draw(|frame| {
                        self.render(frame);
                    })?;

                    if let Some(action) = self.handle_event(ct_event) {
                        self.handle_action(action, backend).await?
                    }
                }
            }
        }

        Ok(())
    }

    async fn update_view_model<B: TuiBackend>(&mut self, backend: &B) -> anyhow::Result<()> {
        let timer_active = backend.get_active_time().await?.is_some();
        match self.current_screen {
            ScreenType::ProjectOverview => {
                let view_model = backend.load_projects().await?;
                self.screens
                    .project_overview
                    .set_view_model(view_model.clone(), timer_active);
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

    async fn update_tick<B: TuiBackend>(&mut self, backend: &B) -> anyhow::Result<()> {
        self.active_timer
            .set_passed_time(backend.get_active_time().await?);

        if self.enforce_vm_update_on_next_tick {
            self.update_view_model(backend).await?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if area.width < 150 || area.height < 40 {
            self.render_terminal_to_small_text(frame, area);
            return;
        }

        if self.welcome_active {
            self.screens.welcome_screen.render(frame, area);
            return;
        }

        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(" Chrono Forge ")
            .title_alignment(Alignment::Center);
        let screen_rect = main_block.inner(area);
        frame.render_widget(main_block, area);

        let app_layout_rects = Layout::vertical([
            Constraint::Length(3), // tab view
            Constraint::Length(1), // separator
            Constraint::Min(1),    // screen
            Constraint::Length(1), // separator
            Constraint::Length(3), // help bar
        ])
        .split(screen_rect);

        self.render_header(frame, app_layout_rects[0]);
        self.render_separator(frame, app_layout_rects[1]);

        let help_text;
        let screen_area = app_layout_rects[2];
        match self.current_screen {
            ScreenType::ProjectOverview => {
                let screen = &mut self.screens.project_overview;
                screen.render(frame, screen_area);
                help_text = screen.get_help_text();
                self.enforce_vm_update_on_next_tick =
                    screen.enforce_view_model_update_on_next_tick();
            }
            ScreenType::Timer => {
                todo!()
            }
            ScreenType::Dashboard => {
                todo!()
            }
        }

        self.render_separator(frame, app_layout_rects[3]);

        // help box
        let help_box = Paragraph::new(Line::from(help_text).alignment(Alignment::Center));
        let mut rect = app_layout_rects[4];
        rect.y += 1;
        frame.render_widget(help_box, rect);
    }

    fn render_terminal_to_small_text(&self, frame: &mut Frame, area: Rect) {
        let vertical_layout = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(area);
        let text = Paragraph::new("Window too small. At least 150x40 required").centered();

        frame.render_widget(text, vertical_layout[1]);
    }

    fn render_header(&self, frame: &mut Frame, rect: Rect) {
        let tab_time_split =
            Layout::horizontal([Constraint::Min(1), Constraint::Length(15)]).split(rect);

        self.active_timer.render(frame, tab_time_split[1]);
    }

    fn render_separator(&self, frame: &mut Frame, area: Rect) {
        let separator = symbols::line::HORIZONTAL.repeat(area.width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = area;
        rect.x = rect.x + 1;
        frame.render_widget(separator_widget, rect);
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
            AppAction::CreateTask(name, project_id) => {
                backend.create_task(name, project_id).await?;
            }
            AppAction::RenameTask(task_id, new_name) => {
                backend.rename_task(task_id, new_name).await?;
            }
            AppAction::DeleteTask(task_id) => {
                backend.delete_task(task_id).await?;
            }
            AppAction::StartTimer(task_id) => {
                backend.start_timer(task_id).await?;
            }
            AppAction::StopTimer => {
                backend.stop_timer().await?;
            }
            AppAction::CreateTimeEntry(task_id, start_time, end_time) => {
                backend
                    .create_time_entry(task_id, start_time, end_time)
                    .await?;
            }
            AppAction::EditTimeEntry(entry_id, start_time, end_time) => {
                backend
                    .edit_time_entry(entry_id, start_time, end_time)
                    .await?;
            }
            AppAction::DeleteTimeEntry(entry_id) => {
                backend.delete_time_entry(entry_id).await?;
            }
            AppAction::AssignTask(task_id, project_id) => {
                backend.assign_task(task_id, project_id).await?;
            }
            AppAction::AssignTimeEntry(time_entry_id, task_id) => {
                backend.assign_time_entry(time_entry_id, task_id).await?;
            }
        }
        Ok(())
    }
}
