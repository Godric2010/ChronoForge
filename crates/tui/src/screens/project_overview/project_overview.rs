use crate::app_action::AppAction;
use crate::screens::project_overview::mode::Mode;
use crate::screens::project_overview::project_overview_view_model::ProjectOverviewViewModel;
use crate::widgets::confirmation_dialog::{ConfirmationDialog, ConfirmationResult};
use crate::widgets::selectable_list::SelectableList;
use crate::widgets::text_edit_dialog::{DialogResult, TextEditDialog};
use crossterm::event::{Event, KeyCode, KeyEvent};
use domain::types::{Project, Task};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use uuid::Uuid;

pub struct ProjectOverviewScreen {
    view_model: ProjectOverviewViewModel,
    mode: Mode,
    projects_list_widget: SelectableList,
    task_list_widget: SelectableList,
    text_edit_dialog: Option<TextEditDialog>,
    confirmation_dialog: Option<ConfirmationDialog>,
    help_text: String,
    timer_active: bool
}

impl ProjectOverviewScreen {
    pub fn new() -> Self {
        let mut this = Self {
            view_model: ProjectOverviewViewModel::default(),
            mode: Mode::ProjectSelection,
            projects_list_widget: SelectableList::default(),
            task_list_widget: SelectableList::default(),
            text_edit_dialog: None,
            confirmation_dialog: None,
            help_text: String::new(),
            timer_active: false
        };
        this.enable_project_selection_mode();
        this
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }
    
    
    pub fn set_view_model(&mut self, view_model: ProjectOverviewViewModel, timer_active: bool) {
        self.view_model = view_model;
        self.timer_active = timer_active;

        let project_names: Vec<String> = self
            .view_model
            .projects
            .iter()
            .map(|p| p.project.name.clone())
            .collect();
        self.projects_list_widget.items = project_names;
        self.projects_list_widget.title = Some("Projects".to_string());

        self.task_list_widget.title = Some("Tasks".to_string());
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::horizontal([
            Constraint::Min(1), // projects list
            Constraint::Min(1), // task list
            Constraint::Min(1), // start/stop timer question
        ])
        .split(area);

        // project list
        self.projects_list_widget.render(frame, chunks[0]);

        // task list
        if let Some(selected_project) = &self.get_selected_project() {
            self.fill_task_list(&selected_project.id);
            self.task_list_widget.render(frame, chunks[1]);
        }

        // start/stop task

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
                Mode::TaskSelection => self.handle_task_selection_events(key_event),
                Mode::TaskCreation => self.handle_task_creation_events(key_event),
                Mode::TaskEdit => self.handle_task_editing_events(key_event),
                Mode::TaskDeletion => self.handle_task_deletion_events(key_event),
                Mode::StartStopTimer => None,
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
                    if let Some(project) = &self.get_selected_project() {
                        self.mode = Mode::ProjectEdit;
                        self.build_text_dialog(project.name.clone(), "Rename the project");
                    }
                    None
                }
                'd' => {
                    if let Some(project) = &self.get_selected_project() {
                        self.mode = Mode::ProjectDeletion;
                        self.confirmation_dialog = Some(ConfirmationDialog::new(format!(
                            "Delete project \"{}\"?",
                            project.name
                        )));
                    }
                    None
                }
                _ => None,
            },
            KeyCode::Right => {
                if let Some(_) = &self.get_selected_project() {
                    self.enable_task_selection_mode();
                }
                None
            }
            _ => {
                self.projects_list_widget.handle_event(&key_event);
                None
            }
        }
    }
    fn handle_task_selection_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        match key_event.code {
            KeyCode::Esc => Some(AppAction::Quit),
            KeyCode::Char(c) => match c {
                'q' => Some(AppAction::Quit),
                'n' => {
                    self.mode = Mode::TaskCreation;
                    self.build_text_dialog(String::new(), "Create new task");
                    None
                }
                'e' => {
                    if let Some(task) = &self.get_selected_task() {
                        self.mode = Mode::TaskEdit;
                        self.build_text_dialog(task.name.clone(), "Rename the task");
                    }
                    None
                }
                'd' => {
                    if let Some(task) = &self.get_selected_task() {
                        self.mode = Mode::TaskDeletion;
                        self.confirmation_dialog = Some(ConfirmationDialog::new(format!(
                            "Delete task \"{}\"?",
                            task.name
                        )));
                    }
                    None
                }
                's' => {
                    if self.timer_active{
                        self.timer_active = false;
                        return Some(AppAction::StopTimer)
                    }

                    if let Some(task) = &self.get_selected_task() {
                        self.timer_active = true;
                        return Some(AppAction::StartTimer(task.id));
                    }
                    None
                }
                _ => None,
            },
            KeyCode::Left => {
                self.enable_project_selection_mode();
                None
            }
            KeyCode::Right => {
                if let Some(task) = &self.get_selected_task() {
                    // self.mode = Mode::StartStopTimer;

                    // Enable start/stop screen
                }
                None
            }
            _ => {
                self.task_list_widget.handle_event(&key_event);
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
    fn handle_task_creation_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            let dialog_result = text_edit_dialog.handle_event(&key_event);
            return match dialog_result {
                DialogResult::None => None,
                DialogResult::Confirmed(text) => {
                    let project_id = self.get_selected_project()?.id;
                    self.enable_task_selection_mode();
                    Some(AppAction::CreateTask(text, project_id))
                }
                DialogResult::Cancelled => {
                    self.enable_task_selection_mode();
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
                    let project = self.get_selected_project();

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
    fn handle_task_editing_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            let dialog_result = text_edit_dialog.handle_event(&key_event);
            return match dialog_result {
                DialogResult::None => None,
                DialogResult::Confirmed(text) => {
                    let task = self.get_selected_task();

                    self.enable_task_selection_mode();
                    if let Some(task) = task {
                        return Some(AppAction::RenameTask(task.id, text));
                    };
                    panic!("Try to edit project, but no object is selected!")
                }
                DialogResult::Cancelled => {
                    self.enable_task_selection_mode();
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
                    let project = self.get_selected_project();
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
    fn handle_task_deletion_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(confirmation_dialog) = &mut self.confirmation_dialog {
            let result = confirmation_dialog.handle_event(&key_event);
            return match result {
                ConfirmationResult::None => None,
                ConfirmationResult::Confirmed => {
                    let task = self.get_selected_task();
                    self.enable_task_selection_mode();
                    if let Some(task) = task {
                        return Some(AppAction::DeleteTask(task.id));
                    }
                    None
                }
                ConfirmationResult::Cancelled => {
                    self.enable_task_selection_mode();
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
        self.projects_list_widget.highlight = true;
        self.task_list_widget.highlight = false;
        self.confirmation_dialog = None;
        self.text_edit_dialog = None;
        self.help_text =
            "[N]ew project | [E]dit project | [D]elete project | <Right>: Go to tasks | <Up/Down>"
                .to_string();
    }

    fn enable_task_selection_mode(&mut self) {
        self.mode = Mode::TaskSelection;
        self.projects_list_widget.highlight = false;
        self.task_list_widget.highlight = true;
        self.confirmation_dialog = None;
        self.text_edit_dialog = None;
        self.help_text = "[N]ew task | [E]dit task | [D]elete task | [S]tart/[S]top timer | <Left>: Go to projects | <Right>: Start/Stop timer | <Up/Down>".to_string();
    }

    fn get_selected_project(&self) -> Option<Project> {
        let index = self.projects_list_widget.get_selected_index()?;
        let project_vm = self.view_model.projects.get(index)?;
        Some(project_vm.project.clone())
    }
    fn get_selected_task(&self) -> Option<Task> {
        let project_index = self.projects_list_widget.get_selected_index()?;
        let task_index = self.task_list_widget.get_selected_index()?;
        let task_vm = &self.view_model.projects[project_index].tasks[task_index];
        Some(task_vm.task.clone())
    }

    fn fill_task_list(&mut self, project_id: &Uuid) {
        let vm = &self
            .view_model
            .projects
            .iter()
            .find(|p_vm| p_vm.project.id == project_id.clone())
            .unwrap();

        let tasks = &vm.tasks;
        let task_names: Vec<String> = tasks.iter().map(|t| t.task.name.clone()).collect();
        self.task_list_widget.items = task_names;
    }
}
