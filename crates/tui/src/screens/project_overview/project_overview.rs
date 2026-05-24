use crate::app_action::AppAction;
use crate::screens::project_overview::mode::Mode;
use crate::screens::project_overview::project_overview_view_model::ProjectOverviewViewModel;
use crate::widgets::confirmation_dialog::{ConfirmationDialog, ConfirmationResult};
use crate::widgets::list_dialog;
use crate::widgets::list_dialog::{ListDialog, ListItem};
use crate::widgets::selectable_card_list::project_card::ProjectCard;
use crate::widgets::selectable_card_list::task_card::TaskCard;
use crate::widgets::selectable_card_list::time_entry_card::TimeEntryCard;
use crate::widgets::selectable_card_list::SelectableCardList;
use crate::widgets::text_edit_dialog::{DialogResult, TextEditDialog};
use crate::widgets::time_edit_dialog::{EditResult, TimeEditDialog};
use chrono::Utc;
use crossterm::event::{Event, KeyCode, KeyEvent};
use domain::types::{Project, Task, TimeEntry};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use uuid::Uuid;

pub struct ProjectOverviewScreen {
    view_model: ProjectOverviewViewModel,
    mode: Mode,
    projects_list_widget: SelectableCardList<ProjectCard>,
    task_list_widget: SelectableCardList<TaskCard>,
    time_entry_widget: SelectableCardList<TimeEntryCard>,
    text_edit_dialog: Option<TextEditDialog>,
    confirmation_dialog: Option<ConfirmationDialog>,
    time_edit_dialog: Option<TimeEditDialog>,
    list_dialog: Option<ListDialog>,
    help_text: String,
    enforce_view_model_update_on_next_tick: bool,
    timer_active: bool,
}

impl ProjectOverviewScreen {
    pub fn new() -> Self {
        let mut this = Self {
            view_model: ProjectOverviewViewModel::default(),
            mode: Mode::ProjectSelection,
            projects_list_widget: SelectableCardList::<ProjectCard>::default(),
            task_list_widget: SelectableCardList::<TaskCard>::default(),
            time_entry_widget: SelectableCardList::<TimeEntryCard>::default(),
            text_edit_dialog: None,
            confirmation_dialog: None,
            time_edit_dialog: None,
            list_dialog: None,
            help_text: String::new(),
            enforce_view_model_update_on_next_tick: false,
            timer_active: false,
        };
        this.enable_project_selection_mode();
        this
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }

    pub fn enforce_view_model_update_on_next_tick(&mut self) -> bool {
        let enforce = self.enforce_view_model_update_on_next_tick;
        if enforce {
            self.enforce_view_model_update_on_next_tick = false;
        }
        enforce
    }

    pub fn set_view_model(&mut self, view_model: ProjectOverviewViewModel, timer_active: bool) {
        self.view_model = view_model;
        self.timer_active = timer_active;

        self.projects_list_widget.title = "Projects".to_string();
        self.fill_projects_list();

        self.task_list_widget.title = "Tasks".to_string();

        self.time_entry_widget.title = "Time Entries".to_string();
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

            // time entry list
            if let Some(selected_task) = &self.get_selected_task() {
                self.fill_time_entry_list(&selected_project.id, &selected_task.id);
                self.time_entry_widget.render(frame, chunks[2]);
            }
        }

        // dialog boxes
        if let Some(text_edit_dialog) = &mut self.text_edit_dialog {
            text_edit_dialog.render(frame, area);
        }

        if let Some(time_edit_dialog) = &mut self.time_edit_dialog {
            time_edit_dialog.render(frame, area);
        }

        if let Some(list_dialog) = &mut self.list_dialog {
            list_dialog.render(frame, area);
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
                Mode::TimeEntrySelection => self.handle_time_entry_selection_events(key_event),
                Mode::TimeEntryCreation => self.handle_time_entry_creation_events(key_event),
                Mode::TimeEntryEdit => self.handle_time_entry_edit_events(key_event),
                Mode::TimeEntryDeletion => self.handle_time_entry_deletion_events(key_event),
                Mode::TaskAssign => self.handle_task_reassignment_events(key_event),
                Mode::TimeEntryAssign => self.handle_time_entry_reassignment_events(key_event),
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
                'a' => {
                    self.mode = Mode::TaskAssign;
                    let list_items = self
                        .view_model
                        .projects
                        .iter()
                        .map(|project| ListItem {
                            name: project.project.name.clone(),
                            id: project.project.id,
                        })
                        .collect::<Vec<ListItem>>();
                    self.list_dialog =
                        Some(ListDialog::new("Assign task to project", list_items, 50));
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
                    if self.timer_active {
                        self.timer_active = false;
                        return Some(AppAction::StopTimer);
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
                if let Some(_) = &self.get_selected_task() {
                    self.enable_time_entry_mode();
                }
                None
            }
            _ => {
                self.task_list_widget.handle_event(&key_event);
                None
            }
        }
    }
    fn handle_time_entry_selection_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        match key_event.code {
            KeyCode::Esc => Some(AppAction::Quit),
            KeyCode::Char(c) => match c {
                'q' => Some(AppAction::Quit),
                'n' => {
                    self.mode = Mode::TimeEntryCreation;
                    self.build_time_edit_dialog("Create new time entry", None);
                    None
                }
                'e' => {
                    if let Some(_) = &self.get_selected_time_entry() {
                        self.mode = Mode::TimeEntryEdit;
                        let time_entry = self.get_selected_time_entry().clone();
                        self.build_time_edit_dialog("Edit time entry", time_entry);
                    }
                    None
                }
                'a' => {
                    self.mode = Mode::TimeEntryAssign;
                    let selected_project_index = self.projects_list_widget.get_selected_index()?;
                    let list_items = self.view_model.projects[selected_project_index]
                        .tasks
                        .iter()
                        .map(|task| ListItem {
                            name: task.task.name.clone(),
                            id: task.task.id,
                        })
                        .collect::<Vec<ListItem>>();
                    self.list_dialog =
                        Some(ListDialog::new("Assign time entry to task", list_items, 50));
                    None
                }
                'd' => {
                    if let Some(_) = &self.get_selected_time_entry() {
                        self.mode = Mode::TimeEntryDeletion;
                        self.confirmation_dialog =
                            Some(ConfirmationDialog::new("Delete time entry?".to_string()));
                    }
                    None
                }
                _ => None,
            },
            KeyCode::Left => {
                self.enable_task_selection_mode();
                None
            }
            _ => {
                self.time_entry_widget.handle_event(&key_event);
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
                    self.enforce_view_model_update_on_next_tick = true;
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
                    self.enforce_view_model_update_on_next_tick = true;
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
    fn handle_time_entry_creation_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(time_edit_dialog) = &mut self.time_edit_dialog {
            let dialog_result = time_edit_dialog.handle_event(&key_event);
            return match dialog_result {
                EditResult::None => None,
                EditResult::Confirmed(start_time, end_time) => {
                    let task_id = self.get_selected_task()?.id;
                    self.enable_task_selection_mode();
                    self.enforce_view_model_update_on_next_tick = true;
                    Some(AppAction::CreateTimeEntry(task_id, start_time, end_time))
                }
                EditResult::Cancelled => {
                    self.enable_time_entry_mode();
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
                        self.enforce_view_model_update_on_next_tick = true;
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
                        self.enforce_view_model_update_on_next_tick = true;
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
    fn handle_time_entry_edit_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(time_edit_dialog) = &mut self.time_edit_dialog {
            let dialog_result = time_edit_dialog.handle_event(&key_event);
            return match dialog_result {
                EditResult::None => None,
                EditResult::Confirmed(start_time, end_time) => {
                    let entry_id = self.get_selected_time_entry()?.id;
                    self.enable_task_selection_mode();
                    self.enforce_view_model_update_on_next_tick = true;
                    Some(AppAction::EditTimeEntry(entry_id, start_time, end_time))
                }
                EditResult::Cancelled => {
                    self.enable_time_entry_mode();
                    None
                }
            };
        }
        None
    }

    fn handle_task_reassignment_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(list_dialog) = &mut self.list_dialog {
            let dialog_result = list_dialog.handle_event(&key_event);
            return match dialog_result {
                list_dialog::DialogResult::None => None,
                list_dialog::DialogResult::Confirmed(project_id) => {
                    let task_id = self.get_selected_task()?.id;
                    self.enable_task_selection_mode();
                    self.enforce_view_model_update_on_next_tick = true;
                    Some(AppAction::AssignTask(task_id, project_id))
                }
                list_dialog::DialogResult::Cancelled => {
                    self.enable_task_selection_mode();
                    None
                }
            };
        }
        None
    }

    fn handle_time_entry_reassignment_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(list_dialog) = &mut self.list_dialog {
            let dialog_result = list_dialog.handle_event(&key_event);
            return match dialog_result {
                list_dialog::DialogResult::None => None,
                list_dialog::DialogResult::Confirmed(task_id) => {
                    let time_entry_id = self.get_selected_time_entry()?.id;
                    self.enable_time_entry_mode();
                    self.enforce_view_model_update_on_next_tick = true;
                    Some(AppAction::AssignTimeEntry(time_entry_id, task_id))
                }
                list_dialog::DialogResult::Cancelled => {
                    self.enable_time_entry_mode();
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
                        self.enforce_view_model_update_on_next_tick = true;
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
                        self.enforce_view_model_update_on_next_tick = true;
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
    fn handle_time_entry_deletion_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(confirmation_dialog) = &mut self.confirmation_dialog {
            let result = confirmation_dialog.handle_event(&key_event);
            return match result {
                ConfirmationResult::None => None,
                ConfirmationResult::Confirmed => {
                    let time_entry = self.get_selected_time_entry();
                    self.enable_time_entry_mode();
                    if let Some(time_entry) = time_entry {
                        self.enforce_view_model_update_on_next_tick = true;
                        return Some(AppAction::DeleteTimeEntry(time_entry.id));
                    }
                    None
                }
                ConfirmationResult::Cancelled => {
                    self.enable_time_entry_mode();
                    None
                }
            };
        }
        None
    }

    fn build_text_dialog(&mut self, content: String, title: &str) {
        self.text_edit_dialog = Some(TextEditDialog::new(title, content, 50));
    }

    fn build_time_edit_dialog(&mut self, title: &str, time_entry: Option<TimeEntry>) {
        let width_percentage = 50;
        if let Some(time_entry) = time_entry {
            self.time_edit_dialog = Some(TimeEditDialog::new(
                title,
                time_entry.start_time,
                time_entry.end_time,
                width_percentage,
            ));
        } else {
            self.time_edit_dialog = Some(TimeEditDialog::new(
                title,
                Utc::now(),
                Utc::now(),
                width_percentage,
            ));
        }
    }

    fn enable_project_selection_mode(&mut self) {
        self.mode = Mode::ProjectSelection;
        self.projects_list_widget.set_active(true, true);
        self.task_list_widget.set_active(false, false);
        self.time_entry_widget.set_active(false, false);
        self.confirmation_dialog = None;
        self.text_edit_dialog = None;
        self.time_edit_dialog = None;
        self.list_dialog = None;
        self.help_text =
            "[N]ew project | [E]dit project | [D]elete project | <Right>: Go to tasks | <Up/Down>"
                .to_string();
    }

    fn enable_task_selection_mode(&mut self) {
        self.mode = Mode::TaskSelection;
        self.projects_list_widget.set_active(false, true);
        self.task_list_widget.set_active(true, true);
        self.time_entry_widget.set_active(false, false);
        self.confirmation_dialog = None;
        self.text_edit_dialog = None;
        self.time_edit_dialog = None;
        self.list_dialog = None;
        self.help_text = "[N]ew task | [E]dit task | [A]ssign to other project | [D]elete task | [S]tart/[S]top timer | <Left>: Go to projects | <Right>: Go to Time Entries | <Up/Down>".to_string();
    }

    fn enable_time_entry_mode(&mut self) {
        self.mode = Mode::TimeEntrySelection;
        self.projects_list_widget.set_active(false, true);
        self.task_list_widget.set_active(false, true);
        self.time_entry_widget.set_active(true, true);
        self.confirmation_dialog = None;
        self.text_edit_dialog = None;
        self.time_edit_dialog = None;
        self.list_dialog = None;
        self.help_text = "[N]ew time entry | [E]dit time entry | [A]ssign to other task | [D]elete time entry | <Left>: Go to tasks | <Up/Down>".to_string();
    }

    fn get_selected_project(&mut self) -> Option<Project> {
        let index = self.projects_list_widget.get_selected_index()?;
        let project_vm = self.view_model.projects.get(index)?;
        Some(project_vm.project.clone())
    }
    fn get_selected_task(&mut self) -> Option<Task> {
        let project_index = self.projects_list_widget.get_selected_index()?;
        let task_index = self.task_list_widget.get_selected_index()?;
        let task_vm = &self.view_model.projects[project_index].tasks[task_index];
        Some(task_vm.task.clone())
    }

    fn get_selected_time_entry(&mut self) -> Option<TimeEntry> {
        let project_index = self.projects_list_widget.get_selected_index()?;
        let task_index = self.task_list_widget.get_selected_index()?;
        let entry_index = self.time_entry_widget.get_selected_index()?;

        let entry_vm =
            &self.view_model.projects[project_index].tasks[task_index].time_entries[entry_index];
        Some(entry_vm.time_entry.clone())
    }

    fn fill_projects_list(&mut self) {
        let project_cards: Vec<ProjectCard> = self
            .view_model
            .projects
            .iter()
            .map(|p| {
                ProjectCard::new(
                    p.project.name.clone(),
                    p.tasks.len(),
                    p.total_project_time_min,
                )
            })
            .collect();
        self.projects_list_widget.item_height = 7;
        self.projects_list_widget.cards = project_cards;
    }

    fn fill_task_list(&mut self, project_id: &Uuid) {
        let vm = &self
            .view_model
            .projects
            .iter()
            .find(|p_vm| p_vm.project.id == project_id.clone())
            .unwrap();

        let tasks = &vm.tasks;
        let task_cards: Vec<TaskCard> = tasks
            .iter()
            .map(|t| TaskCard::new(t.task.name.clone(), t.total_task_time_min))
            .collect();
        self.task_list_widget.item_height = 5;
        self.task_list_widget.cards = task_cards;
    }

    fn fill_time_entry_list(&mut self, project_id: &Uuid, task_id: &Uuid) {
        let project_vm = self
            .view_model
            .projects
            .iter()
            .find(|p_vm| p_vm.project.id == project_id.clone())
            .unwrap();

        let tasks = &project_vm.tasks;
        let task_vm = tasks.iter().find(|t| t.task.id == task_id.clone()).unwrap();

        let mut entries = task_vm.time_entries.clone();
        entries.sort_by(|a, b| b.end_time.cmp(&a.end_time));

        let time_entry_cards: Vec<TimeEntryCard> = task_vm
            .time_entries
            .iter()
            .map(|te| TimeEntryCard::new(te.start_time, te.end_time))
            .collect();

        self.time_entry_widget.item_height = 6;
        self.time_entry_widget.cards = time_entry_cards;
    }
}
