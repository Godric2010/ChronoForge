use crate::app_action::AppAction;
use crate::screens::project_overview::project_overview_view_model::ProjectOverviewViewModel;
use crate::widgets::selectable_list::SelectableList;
use crate::widgets::text_edit_dialog::{DialogResult, TextEditDialog};
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct ProjectOverviewScreen {
    view_model: ProjectOverviewViewModel,
    projects_list_widget: SelectableList,
    text_edit_dialog: Option<TextEditDialog>,
}

impl ProjectOverviewScreen {
    pub fn new() -> Self {
        Self {
            view_model: ProjectOverviewViewModel::default(),
            projects_list_widget: SelectableList::default(),
            text_edit_dialog: None,
        }
    }

    pub fn set_view_model(&mut self, view_model: ProjectOverviewViewModel) {
        self.view_model = view_model;

        let project_names: Vec<String> = self
            .view_model
            .projects
            .iter()
            .map(|p| p.name.clone())
            .collect();
        self.projects_list_widget.items = project_names;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

        // Header
        let header = Block::default().title("Projects").borders(Borders::ALL);
        frame.render_widget(header, chunks[0]);

        // project list
        self.projects_list_widget.render(frame, chunks[1]);

        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            text_edit_dialog.render(frame, area);
        }

        // Footer
        let footer = Block::default()
            .title("Enter something useful here")
            .borders(Borders::ALL);
        frame.render_widget(footer, chunks[2]);
    }

    pub fn handle_event(&mut self, event: Event) -> Option<AppAction> {
        if let Some(key_event) = event.as_key_event() {
            if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
                let dialog_result = text_edit_dialog.handle_event(&key_event);
                return match dialog_result {
                    DialogResult::None => None,
                    DialogResult::Confirmed(text) => {
                        self.text_edit_dialog = None;
                        Some(AppAction::CreateProject(text))
                    },
                    DialogResult::Cancelled => {
                        self.text_edit_dialog = None;
                        None
                    }
                };
            }

            return match key_event.code {
                KeyCode::Esc => Some(AppAction::Quit),
                KeyCode::Char(c) => match c {
                    'n' => {
                        self.build_text_dialog();
                        None
                    }
                    _ => None
                }
                _ => {
                    self.projects_list_widget.handle_event(&key_event);
                    None
                }
            };
        }
        None
    }

    fn build_text_dialog(&mut self) {
        self.text_edit_dialog = Some(TextEditDialog::new(
            "Set project name",
            String::new(),
            50,
        ));
    }
}
