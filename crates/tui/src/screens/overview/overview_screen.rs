use crate::app_action::AppAction;
use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::HelpProvider;
use crate::screens::dialog::help_dialog::HelpDialog;
use crate::screens::overview::mode::{Mode, OverviewGeneralActions};
use crate::screens::overview::overview_dialog::{OverviewDialog, OverviewDialogResult};
use crate::screens::overview::overview_input_maps::create_general_input_map;
use crate::screens::overview::overview_view_model::OverviewViewModel;
use crate::screens::overview::projects_view::ProjectsView;
use crate::screens::overview::tasks_view::TasksView;
use crate::screens::overview::time_entry_view::TimeEntryView;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

pub struct OverviewScreen {
    input_map: InputMap<OverviewGeneralActions>,
    view_model: OverviewViewModel,
    mode: Mode,
    projects_view: ProjectsView,
    tasks_view: TasksView,
    time_entry_view: TimeEntryView,
    overview_dialog: Option<OverviewDialog>,
    help_dialog: Option<HelpDialog>,
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
            input_map: create_general_input_map(),
            view_model: OverviewViewModel::default(),
            mode: Mode::Projects,
            projects_view: ProjectsView::new(),
            tasks_view: TasksView::new(),
            time_entry_view: TimeEntryView::new(),
            overview_dialog: None,
            help_dialog: None,
            help_text: String::new(),
            enforce_view_model_update_on_next_tick: false,
            timer_active: false,
        };
        this.enable_project_selection_mode();
        this
    }

    pub fn get_footer_help_text(&self) -> String {
        let mut footer = match self.mode {
            Mode::Projects => self.projects_view.footer_help(),
            Mode::Tasks => self.tasks_view.footer_help(),
            Mode::TimeEntries => self.time_entry_view.footer_help(),
        };
        self.input_map.append_footer_help(&mut footer);
        KeyBindingHelpContext::build_single_line(footer)
    }

    fn enable_help_dialog(&mut self) {
        let mut help_contexts = match self.mode {
            Mode::Projects => self.projects_view.general_help(),
            Mode::Tasks => self.tasks_view.general_help(),
            Mode::TimeEntries => self.time_entry_view.general_help(),
        };
        self.input_map.append_general_help(&mut help_contexts);

        self.help_dialog = Some(HelpDialog::new(help_contexts));
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

        self.projects_view
            .set_data_from_view_model(&self.view_model);
        self.tasks_view.set_data_from_view_model(&self.view_model);
        self.time_entry_view
            .set_data_from_view_model(&self.view_model);

        if let Some(selected_project) = self.projects_view.get_selected_project() {
            self.tasks_view.update_task_list(selected_project.id);
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::horizontal([
            Constraint::Min(1), // projects list
            Constraint::Min(1), // task list
            Constraint::Min(1), // start/stop timer question
        ])
        .split(area);

        // project list
        self.projects_view.render(frame, chunks[0]);

        // task list
        if let Some(selected_project) = &self.projects_view.get_selected_project() {
            self.tasks_view.set_selected_project(selected_project.id);
            self.tasks_view.render(frame, chunks[1]);

            // time entry list
            if let Some(selected_task) = &self.tasks_view.get_selected_task() {
                self.time_entry_view.set_selected_task(selected_task.id);
                self.time_entry_view.render(frame, chunks[2]);
            }
        }

        if let Some(overview_dialog) = &mut self.overview_dialog {
            overview_dialog.render(frame, area);
        }

        if let Some(help_dialog) = &self.help_dialog {
            help_dialog.render(frame, area);
        }
    }

    pub fn handle_event(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(help_dialog) = self.help_dialog.as_mut() {
            let help_shall_close = help_dialog.handle_key(key_event);
            if help_shall_close {
                self.help_dialog = None;
            }

            return None;
        }

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
                OverviewDialogResult::Help(help_context) => {
                    self.help_dialog = Some(HelpDialog::new(help_context));
                    None
                }
            };
        }

        let action = self.input_map.find_action(key_event);
        if let Some(action) = action {
            return match action {
                OverviewGeneralActions::Quit => Some(AppAction::Quit),
                OverviewGeneralActions::NextMode => {
                    self.set_next_mode();
                    None
                }
                OverviewGeneralActions::PrevMode => {
                    self.set_previous_mode();
                    None
                }
                OverviewGeneralActions::ToggleTimer => self.toggle_timer(),
                OverviewGeneralActions::Help => {
                    self.enable_help_dialog();
                    None
                }
            };
        }

        match self.mode {
            Mode::Projects => {
                self.overview_dialog = self.projects_view.handle_input(key_event);
                None
            }
            Mode::Tasks => {
                self.overview_dialog = self.tasks_view.handle_input(key_event);
                None
            }
            Mode::TimeEntries => {
                self.overview_dialog = self.time_entry_view.handle_input(key_event);
                None
            }
        }
    }

    fn set_next_mode(&mut self) {
        match self.mode {
            Mode::Projects => self.enable_task_selection_mode(),
            Mode::Tasks => self.enable_time_entry_mode(),
            Mode::TimeEntries => {}
        }
    }

    fn set_previous_mode(&mut self) {
        match self.mode {
            Mode::Projects => {}
            Mode::Tasks => self.enable_project_selection_mode(),
            Mode::TimeEntries => self.enable_task_selection_mode(),
        }
    }

    fn toggle_timer(&mut self) -> Option<AppAction> {
        if self.timer_active {
            self.timer_active = false;
            return Some(AppAction::StopTimer);
        }

        if let Some(task) = self.tasks_view.get_selected_task() {
            self.timer_active = true;
            return Some(AppAction::StartTimer(task.id));
        }
        None
    }

    fn enable_project_selection_mode(&mut self) {
        self.mode = Mode::Projects;
        self.projects_view.set_active(true, true);
        self.tasks_view.set_active(false, false);
        self.time_entry_view.set_active(false);
        self.help_text =
            "[N]ew project | [E]dit project | [D]elete project | <Right>: Go to tasks | <Up/Down>"
                .to_string();
    }

    fn enable_task_selection_mode(&mut self) {
        self.mode = Mode::Tasks;
        self.projects_view.set_active(false, true);
        self.tasks_view.set_active(true, true);
        self.time_entry_view.set_active(false);
        self.help_text = "[N]ew task | [E]dit task | [A]ssign to other project | [D]elete task | [S]tart/[S]top timer | <Left>: Go to projects | <Right>: Go to Time Entries | <Up/Down>".to_string();
    }

    fn enable_time_entry_mode(&mut self) {
        self.mode = Mode::TimeEntries;
        self.projects_view.set_active(false, true);
        self.tasks_view.set_active(false, true);
        self.time_entry_view.set_active(true);
        self.help_text = "[N]ew time entry | [E]dit time entry | [A]ssign to other task | [D]elete time entry | <Left>: Go to tasks | <Up/Down>".to_string();
    }
}
