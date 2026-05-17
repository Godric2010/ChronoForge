use crate::app_action::AppAction;
use crate::screens::project_overview::mode::Mode;
use crate::screens::project_overview::project_overview_view_model::ProjectOverviewViewModel;
use crate::widgets::confirmation_dialog::{ConfirmationDialog, ConfirmationResult};
use crate::widgets::selectable_list::SelectableList;
use crate::widgets::text_edit_dialog::{DialogResult, TextEditDialog};
use crossterm::event::{Event, KeyCode, KeyEvent};
use domain::types::Project;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct ProjectOverviewScreen {
    view_model: ProjectOverviewViewModel,
    mode: Mode,
    projects_list_widget: SelectableList,
    text_edit_dialog: Option<TextEditDialog>,
    confirmation_dialog: Option<ConfirmationDialog>,
    selected_project: Option<Project>,
}

impl ProjectOverviewScreen {
    pub fn new() -> Self {
        Self {
            view_model: ProjectOverviewViewModel::default(),
            mode: Mode::ProjectSelection,
            projects_list_widget: SelectableList::default(),
            text_edit_dialog: None,
            confirmation_dialog: None,
            selected_project: None,
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

        if let Some(confirmation_dialog) = &mut self.confirmation_dialog {
            confirmation_dialog.render(frame, area);
        }

        // Footer
        let footer = Block::default()
            .title("Enter something useful here")
            .borders(Borders::ALL);
        frame.render_widget(footer, chunks[2]);
    }

    pub fn handle_event(&mut self, event: Event) -> Option<AppAction> {
        if let Some(key_event) = event.as_key_event() {
            return match self.mode {
                Mode::ProjectSelection => self.handle_project_selection_events(key_event),
                Mode::ProjectCreation => self.handle_project_creation_events(key_event),
                Mode::ProjectEdit => self.handle_project_editing_events(key_event),
                Mode::ProjectDeletion => self.handle_project_deletion_events(key_event),
            };
        }
        None
    }

    fn handle_project_selection_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        match key_event.code {
            KeyCode::Esc => Some(AppAction::Quit),
            KeyCode::Char(c) => match c {
                'q' => Some(AppAction::Quit),
                'n' => {
                    self.mode = Mode::ProjectCreation;
                    self.build_text_dialog(String::new(), "Create new project");
                    None
                }
                'e' => {
                    self.mode = Mode::ProjectEdit;
                    let index = self.projects_list_widget.get_selected_index();
                    let project = &self.view_model.projects[index];
                    self.selected_project = Some(project.clone());
                    self.build_text_dialog(project.name.clone(), "Rename the project");
                    None
                }
                'd' => {
                    self.mode = Mode::ProjectDeletion;
                    let index = self.projects_list_widget.get_selected_index();
                    let project = &self.view_model.projects[index];
                    self.selected_project = Some(project.clone());
                    self.confirmation_dialog = Some(ConfirmationDialog::new(format!(
                        "Delete project \"{}\"?",
                        project.name
                    )));
                    None
                }
                _ => None,
            },
            _ => {
                self.projects_list_widget.handle_event(&key_event);
                None
            }
        }
    }

    fn handle_project_creation_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            let dialog_result = text_edit_dialog.handle_event(&key_event);
            return match dialog_result {
                DialogResult::None => None,
                DialogResult::Confirmed(text) => {
                    self.text_edit_dialog = None;
                    self.mode = Mode::ProjectSelection;
                    Some(AppAction::CreateProject(text))
                }
                DialogResult::Cancelled => {
                    self.mode = Mode::ProjectSelection;
                    self.text_edit_dialog = None;
                    None
                }
            };
        }
        None
    }

    fn handle_project_editing_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            let dialog_result = text_edit_dialog.handle_event(&key_event);
            return match dialog_result {
                DialogResult::None => None,
                DialogResult::Confirmed(text) => {
                    self.text_edit_dialog = None;
                    let project = self.selected_project.clone();
                    self.selected_project = None;
                    self.mode = Mode::ProjectSelection;
                    if let Some(selected_project) = project {
                        return Some(AppAction::RenameProject(selected_project.id, text));
                    };
                    self.selected_project = None;
                    panic!("Try to edit project, but no object is selected!")
                }
                DialogResult::Cancelled => {
                    self.text_edit_dialog = None;
                    self.selected_project = None;
                    self.mode = Mode::ProjectSelection;
                    None
                }
            };
        }
        None
    }

    fn handle_project_deletion_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(confirmation_dialog) = &mut self.confirmation_dialog {
            let result = confirmation_dialog.handle_event(&key_event);
            return match result {
                ConfirmationResult::None => None,
                ConfirmationResult::Confirmed => {
                    self.confirmation_dialog = None;
                    self.mode = Mode::ProjectSelection;

                    let project = self.selected_project.clone();
                    self.selected_project = None;

                    if let Some(selected_project) = project {
                        return Some(AppAction::DeleteProject(selected_project.id));
                    }
                    None
                }
                ConfirmationResult::Cancelled => {
                    self.confirmation_dialog = None;
                    self.mode = Mode::ProjectSelection;
                    self.selected_project = None;
                    None
                }
            };
        }
        None
    }

    fn build_text_dialog(&mut self, content: String, title: &str) {
        self.text_edit_dialog = Some(TextEditDialog::new(title, content, 50));
    }
}
