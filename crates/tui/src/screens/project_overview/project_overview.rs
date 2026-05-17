use crate::app_action::AppAction;
use crate::screens::project_overview::mode::Mode;
use crate::screens::project_overview::project_overview_view_model::ProjectOverviewViewModel;
use crate::widgets::confirmation_dialog::{ConfirmationDialog, ConfirmationResult};
use crate::widgets::selectable_list::SelectableList;
use crate::widgets::text_edit_dialog::{DialogResult, TextEditDialog};
use crossterm::event::{Event, KeyCode, KeyEvent};
use domain::types::Project;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct ProjectOverviewScreen {
    view_model: ProjectOverviewViewModel,
    mode: Mode,
    projects_list_widget: SelectableList,
    task_list_widget: SelectableList,
    text_edit_dialog: Option<TextEditDialog>,
    confirmation_dialog: Option<ConfirmationDialog>,
    selected_project: Option<Project>,
    help_text: String,
}

impl ProjectOverviewScreen {
    pub fn new() -> Self {
       let mut this =  Self {
            view_model: ProjectOverviewViewModel::default(),
            mode: Mode::ProjectSelection,
            projects_list_widget: SelectableList::default(),
            task_list_widget: SelectableList::default(),
            text_edit_dialog: None,
            confirmation_dialog: None,
            selected_project: None,
            help_text: String::new(),
        };
        this.enable_project_selection_mode();
        this
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }

    pub fn set_view_model(&mut self, view_model: ProjectOverviewViewModel) {
        self.view_model = view_model;

        let project_names: Vec<String> = self
            .view_model
            .projects
            .iter()
            .map(|p| p.project.name.clone())
            .collect();
        self.projects_list_widget.items = project_names;
        self.projects_list_widget.title = Some("Projects".to_string());
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {

        let chunks = Layout::horizontal([
            Constraint::Min(1), // projects list
            Constraint::Min(1), // task list
            Constraint::Min(1), // start/stop timer question
        ]).split(area);


        // project list
        self.projects_list_widget.render(frame, chunks[0]);

        // task list
        self.task_list_widget.render(frame, chunks[1]);

        // dialog boxes
        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            text_edit_dialog.render(frame, area);
        }

        if let Some(confirmation_dialog) = &mut self.confirmation_dialog {
            confirmation_dialog.render(frame, area);
        }
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
                    let project_vm = &self.view_model.projects[index];
                    self.selected_project = Some(project_vm.project.clone());
                    self.build_text_dialog(project_vm.project.name.clone(), "Rename the project");
                    None
                }
                'd' => {
                    self.mode = Mode::ProjectDeletion;
                    let index = self.projects_list_widget.get_selected_index();
                    let project_vm = &self.view_model.projects[index];
                    self.selected_project = Some(project_vm.project.clone());
                    self.confirmation_dialog = Some(ConfirmationDialog::new(format!(
                        "Delete project \"{}\"?",
                        project_vm.project.name
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
                    self.enable_project_selection_mode();
                    Some(AppAction::CreateProject(text))
                }
                DialogResult::Cancelled => {
                    self.enable_project_selection_mode();
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
                    let project = self.selected_project.clone();

                    self.enable_project_selection_mode();
                    if let Some(selected_project) = project {
                        return Some(AppAction::RenameProject(selected_project.id, text));
                    };
                    panic!("Try to edit project, but no object is selected!")
                }
                DialogResult::Cancelled => {
                    self.enable_project_selection_mode();
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
                    let project = self.selected_project.clone();
                    self.enable_project_selection_mode();
                    if let Some(selected_project) = project {
                        return Some(AppAction::DeleteProject(selected_project.id));
                    }
                    None
                }
                ConfirmationResult::Cancelled => {
                    self.enable_project_selection_mode();
                    None
                }
            };
        }
        None
    }

    fn build_text_dialog(&mut self, content: String, title: &str) {
        self.text_edit_dialog = Some(TextEditDialog::new(title, content, 50));
    }

    fn enable_project_selection_mode(&mut self) {
        self.mode = Mode::ProjectSelection;
        self.selected_project = None;
        self.confirmation_dialog = None;
        self.text_edit_dialog = None;
        self.help_text =
            "[N]ew project | [E]dit project | [D]elete project | <Space>: Select | <Up/Down>"
                .to_string();
    }
}
