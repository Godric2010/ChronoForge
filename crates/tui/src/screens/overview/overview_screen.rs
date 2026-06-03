use crate::app_action::AppAction;
use crate::screens::dialog::Dialog;
use crate::screens::overview::mode::Mode;
use crate::screens::overview::overview_dialog::{OverviewDialog, OverviewDialogResult};
use crate::screens::overview::overview_view_model::OverviewViewModel;
use crate::widgets::dialog_widgets::{
    ListItem, ListWidget, TextInputMode, TextInputWidget, TimeEntryWidget, YesNoWidget,
};
use crate::widgets::selectable_card_list::project_card::ProjectCard;
use crate::widgets::selectable_card_list::task_card::TaskCard;
use crate::widgets::selectable_card_list::time_entry_card::TimeEntryCard;
use crate::widgets::selectable_card_list::SelectableCardList;
use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent};
use domain::types::{Project, Task, TimeEntry};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use uuid::Uuid;

pub struct OverviewScreen {
    view_model: OverviewViewModel,
    mode: Mode,
    projects_list_widget: SelectableCardList<ProjectCard>,
    task_list_widget: SelectableCardList<TaskCard>,
    time_entry_widget: SelectableCardList<TimeEntryCard>,
    overview_dialog: Option<OverviewDialog>,
    help_text: String,
    enforce_view_model_update_on_next_tick: bool,
    timer_active: bool,
}
impl Default for OverviewScreen {
    fn default() -> Self {
        Self::new()
    }
}
impl OverviewScreen {
    pub fn new() -> Self {
        let mut this = Self {
            view_model: OverviewViewModel::default(),
            mode: Mode::Projects,
            projects_list_widget: SelectableCardList::<ProjectCard>::default(),
            task_list_widget: SelectableCardList::<TaskCard>::default(),
            time_entry_widget: SelectableCardList::<TimeEntryCard>::default(),
            overview_dialog: None,
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

    pub fn set_view_model(&mut self, view_model: OverviewViewModel, timer_active: bool) {
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

        if let Some(overview_dialog) = &mut self.overview_dialog {
            overview_dialog.render(frame, area);
        }
    }

    pub fn handle_event(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(overview_dialog) = self.overview_dialog.as_mut() {
            let overview_dialog_result = overview_dialog.handle_input(key_event);
            return match overview_dialog_result {
                OverviewDialogResult::None => None,
                OverviewDialogResult::Cancelled => {
                    self.overview_dialog = None;
                    None
                }
                OverviewDialogResult::Confirmed(app_action) => {
                    self.overview_dialog = None;
                    self.enforce_view_model_update_on_next_tick = true;
                    Some(app_action)
                }
            };
        }

        match self.mode {
            Mode::Projects => self.handle_project_selection_events(key_event),
            Mode::Tasks => self.handle_task_selection_events(key_event),
            Mode::TimeEntries => self.handle_time_entry_selection_events(key_event),
        }
    }

    fn handle_project_selection_events(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        match key_event.code {
            KeyCode::Esc => Some(AppAction::Quit),
            KeyCode::Char(c) => match c {
                'q' => Some(AppAction::Quit),
                'n' => {
                    let widget = TextInputWidget::new(TextInputMode::Naming, None);
                    let dialog = Dialog::new("Create new project", widget);
                    self.overview_dialog = Some(OverviewDialog::CreateProject(dialog));
                    None
                }
                'e' => {
                    if let Some(project) = &self.get_selected_project() {
                        let widget =
                            TextInputWidget::new(TextInputMode::Naming, Some(project.name.clone()));
                        let dialog = Dialog::new("Rename the project", widget);
                        self.overview_dialog =
                            Some(OverviewDialog::EditProjectName(dialog, project.id));
                    }
                    None
                }
                'd' => {
                    if let Some(project) = &self.get_selected_project() {
                        let widget = YesNoWidget::new();
                        let dialog = Dialog::new(
                            format!("Delete project \"{}\"?", project.name).as_str(),
                            widget,
                        );
                        self.overview_dialog =
                            Some(OverviewDialog::DeleteProject(dialog, project.id));
                    }
                    None
                }
                _ => None,
            },
            KeyCode::Right => {
                if self.get_selected_project().is_some() {
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
                    let selected_project = self.get_selected_project()?.id;
                    let widget = TextInputWidget::new(TextInputMode::Naming, None);
                    let dialog = Dialog::new("Create new task", widget);
                    self.overview_dialog =
                        Some(OverviewDialog::CreateTask(dialog, selected_project));
                    None
                }
                'e' => {
                    if let Some(task) = &self.get_selected_task() {
                        let widget =
                            TextInputWidget::new(TextInputMode::Naming, Some(task.name.clone()));
                        let dialog = Dialog::new("Rename the task", widget);
                        self.overview_dialog = Some(OverviewDialog::EditTask(dialog, task.id));
                    }
                    None
                }
                'a' => {
                    let list_items = self
                        .view_model
                        .projects
                        .iter()
                        .map(|project| ListItem {
                            name: project.project.name.clone(),
                            id: project.project.id,
                        })
                        .collect::<Vec<ListItem>>();

                    let task_id = self.get_selected_task()?.id;

                    let widget = ListWidget::new(list_items);
                    let dialog = Dialog::new("Assign task to project", widget);
                    self.overview_dialog = Some(OverviewDialog::AssignTask(dialog, task_id));
                    None
                }
                'd' => {
                    if let Some(task) = &self.get_selected_task() {
                        let widget = YesNoWidget::new();
                        let dialog =
                            Dialog::new(format!("Delete task \"{}\"?", task.name).as_str(), widget);
                        self.overview_dialog = Some(OverviewDialog::DeleteTask(dialog, task.id));
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
                if self.get_selected_task().is_some() {
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
                    let selected_task_id = self.get_selected_task()?.id;
                    let widget = TimeEntryWidget::new(Utc::now(), Utc::now());
                    let dialog = Dialog::new("Create new time entry", widget);
                    self.overview_dialog =
                        Some(OverviewDialog::CreateTimeEntry(dialog, selected_task_id));
                    None
                }
                'e' => {
                    if let Some(time_entry) = &self.get_selected_time_entry() {
                        let widget =
                            TimeEntryWidget::new(time_entry.start_time, time_entry.end_time);
                        let dialog = Dialog::new("Edit time entry", widget);
                        self.overview_dialog =
                            Some(OverviewDialog::EditTimeEntry(dialog, time_entry.id));
                    }
                    None
                }
                'a' => {
                    let selected_time_entry = self.get_selected_time_entry()?.id;
                    let selected_project_index = self.projects_list_widget.get_selected_index()?;
                    let list_items = self.view_model.projects[selected_project_index]
                        .tasks
                        .iter()
                        .map(|task| ListItem {
                            name: task.task.name.clone(),
                            id: task.task.id,
                        })
                        .collect::<Vec<ListItem>>();

                    let widget = ListWidget::new(list_items);
                    let dialog = Dialog::new("Assign time entry to task", widget);
                    self.overview_dialog =
                        Some(OverviewDialog::AssignTimeEntry(dialog, selected_time_entry));
                    None
                }
                'd' => {
                    if let Some(entry) = &self.get_selected_time_entry() {
                        let widget = YesNoWidget::new();
                        let dialog = Dialog::new("Delete time entry?", widget);
                        self.overview_dialog =
                            Some(OverviewDialog::DeleteTimeEntry(dialog, entry.id));
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

    fn enable_project_selection_mode(&mut self) {
        self.mode = Mode::Projects;
        self.projects_list_widget.set_active(true, true);
        self.task_list_widget.set_active(false, false);
        self.time_entry_widget.set_active(false, false);
        self.help_text =
            "[N]ew project | [E]dit project | [D]elete project | <Right>: Go to tasks | <Up/Down>"
                .to_string();
    }

    fn enable_task_selection_mode(&mut self) {
        self.mode = Mode::Tasks;
        self.projects_list_widget.set_active(false, true);
        self.task_list_widget.set_active(true, true);
        self.time_entry_widget.set_active(false, false);
        self.help_text = "[N]ew task | [E]dit task | [A]ssign to other project | [D]elete task | [S]tart/[S]top timer | <Left>: Go to projects | <Right>: Go to Time Entries | <Up/Down>".to_string();
    }

    fn enable_time_entry_mode(&mut self) {
        self.mode = Mode::TimeEntries;
        self.projects_list_widget.set_active(false, true);
        self.task_list_widget.set_active(false, true);
        self.time_entry_widget.set_active(true, true);
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
                    p.time_limit,
                )
            })
            .collect();
        self.projects_list_widget
            .update_list_items(project_cards, 7);
    }

    fn fill_task_list(&mut self, project_id: &Uuid) {
        let vm = &self
            .view_model
            .projects
            .iter()
            .find(|p_vm| p_vm.project.id == *project_id)
            .unwrap();

        let tasks = &vm.tasks;
        let task_cards: Vec<TaskCard> = tasks
            .iter()
            .map(|t| TaskCard::new(t.task.name.clone(), t.total_task_time_min, t.time_limit))
            .collect();
        self.task_list_widget.update_list_items(task_cards, 7);
    }

    fn fill_time_entry_list(&mut self, project_id: &Uuid, task_id: &Uuid) {
        let project_vm = self
            .view_model
            .projects
            .iter()
            .find(|p_vm| p_vm.project.id == *project_id)
            .unwrap();

        let tasks = &project_vm.tasks;
        let task_vm = tasks.iter().find(|t| t.task.id == *task_id).unwrap();

        let mut entries = task_vm.time_entries.clone();
        entries.sort_by_key(|b| std::cmp::Reverse(b.end_time));

        let time_entry_cards: Vec<TimeEntryCard> = task_vm
            .time_entries
            .iter()
            .map(|te| TimeEntryCard::new(te.start_time, te.end_time))
            .collect();

        self.time_entry_widget
            .update_list_items(time_entry_cards, 6);
    }
}
