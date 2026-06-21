use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::HelpProvider;
use crate::screens::dialog::Dialog;
use crate::screens::overview::mode::ProjectsModeActions;
use crate::screens::overview::overview_dialog::OverviewDialog;
use crate::screens::overview::overview_input_maps::create_project_mode_input_map;
use crate::screens::overview::OverviewViewModel;
use crate::widgets::dialog_widgets::{ProjectEditWidget, YesNoWidget};
use crate::widgets::selectable_card_list::project_card::ProjectCard;
use crate::widgets::selectable_card_list::SelectableCardList;
use crossterm::event::KeyEvent;
use domain::types::Project;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct ProjectsView {
    projects_list_widget: SelectableCardList<ProjectCard>,
    input_map: InputMap<ProjectsModeActions>,
    projects: Vec<Project>,
}

impl ProjectsView {
    pub fn new() -> ProjectsView {
        let projects_list_widget = SelectableCardList::new("Projects");

        ProjectsView {
            projects_list_widget,
            input_map: create_project_mode_input_map(),
            projects: Vec::new(),
        }
    }
    pub fn set_data_from_view_model(&mut self, view_model: &OverviewViewModel) {
        self.projects = view_model
            .projects
            .iter()
            .map(|vm| vm.project.clone())
            .collect();

        let project_cards: Vec<ProjectCard> = view_model
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

    pub fn set_active(&mut self, active: bool, keep_selected_item: bool) {
        self.projects_list_widget
            .set_active(active, keep_selected_item);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.projects_list_widget.render(frame, area);
    }

    pub fn get_selected_project(&mut self) -> Option<Project> {
        let index = self.projects_list_widget.get_selected_index()?;
        let project = self.projects.get(index)?;
        Some(project.clone())
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<OverviewDialog> {
        let action = self.input_map.find_action(key);
        if let Some(action) = action {
            return match action {
                ProjectsModeActions::New => Some(self.open_new_project_dialog()),
                ProjectsModeActions::Edit => self.open_edit_project_dialog(),
                ProjectsModeActions::Delete => self.open_delete_project_dialog(),
            };
        }
        self.projects_list_widget.handle_event(key);
        None
    }

    fn open_new_project_dialog(&self) -> OverviewDialog {
        let widget = ProjectEditWidget::empty();
        let dialog = Dialog::new("Create new project", widget);
        OverviewDialog::CreateProject(dialog)
    }

    fn open_edit_project_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(project) = &self.get_selected_project() {
            let widget = ProjectEditWidget::new(project);
            let dialog = Dialog::new("Edit the project", widget);
            return Some(OverviewDialog::EditProjectName(dialog, project.id));
        }
        None
    }

    fn open_delete_project_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(project) = &self.get_selected_project() {
            let widget = YesNoWidget::new();
            let dialog = Dialog::new(
                format!("Delete project \"{}\"?", project.name).as_str(),
                widget,
            );

            return Some(OverviewDialog::DeleteProject(dialog, project.id));
        }
        None
    }
}

impl HelpProvider for ProjectsView {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
        self.projects_list_widget.append_footer_help(output);
    }
}
