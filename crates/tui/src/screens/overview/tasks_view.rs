use crate::input::input_map::InputMap;
use crate::screens::dialog::Dialog;
use crate::screens::overview::mode::TasksModeActions;
use crate::screens::overview::overview_dialog::OverviewDialog;
use crate::screens::overview::overview_input_maps::create_task_mode_input_map;
use crate::screens::overview::OverviewViewModel;
use crate::widgets::dialog_widgets::{ListItem, ListWidget, TaskEditWidget, YesNoWidget};
use crate::widgets::selectable_card_list::task_card::TaskCard;
use crate::widgets::selectable_card_list::SelectableCardList;
use crossterm::event::KeyEvent;
use domain::types::{Project, Task};
use ratatui::layout::Rect;
use ratatui::Frame;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
struct TaskViewModel {
    pub assignable_projects: Vec<Project>,
    pub tasks: HashMap<Uuid, Vec<(Task, u32)>>,
}

pub struct TasksView {
    task_list_widget: SelectableCardList<TaskCard>,
    input_map: InputMap<TasksModeActions>,
    model: TaskViewModel,
    selected_project: Option<Uuid>,
}

impl TasksView {
    pub fn new() -> Self {
        let mut task_list_widget = SelectableCardList::default();
        task_list_widget.title = "Tasks".to_string();

        Self {
            task_list_widget,
            input_map: create_task_mode_input_map(),
            model: TaskViewModel::default(),
            selected_project: None,
        }
    }
    pub fn set_data_from_view_model(&mut self, view_model: &OverviewViewModel) {
        let mut task_view_model = TaskViewModel::default();
        view_model.projects.iter().for_each(|p_vm| {
            let project_id = p_vm.project.id;
            task_view_model
                .assignable_projects
                .push(p_vm.project.clone());

            let tasks_of_project = p_vm
                .tasks
                .iter()
                .map(|t_vm| (t_vm.task.clone(), t_vm.total_task_time_min))
                .collect::<Vec<_>>();
            task_view_model.tasks.insert(project_id, tasks_of_project);
        });
        self.model = task_view_model;
    }

    pub fn set_selected_project(&mut self, selected_project: Uuid) {
        if let Some(project) = self.selected_project {
            if selected_project == project {
                return;
            }
        }

        self.selected_project = Some(selected_project);
        let selected_tasks = self.model.tasks.get(&selected_project);
        if selected_tasks.is_none() {
            return;
        }
        let selected_tasks = selected_tasks.unwrap();
        let task_cards: Vec<TaskCard> = selected_tasks
            .iter()
            .map(|(task, total_time)| {
                TaskCard::new(task.name.clone(), *total_time, task.time_limit)
            })
            .collect::<Vec<TaskCard>>();
        self.task_list_widget.update_list_items(task_cards, 7);
    }

    pub fn set_active(&mut self, active: bool, keep_selected_item: bool) {
        self.task_list_widget.set_active(active, keep_selected_item);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.task_list_widget.render(frame, area);
    }

    pub fn get_selected_task(&mut self) -> Option<Task> {
        if let Some(selected_project) = self.selected_project {
            let index = self.task_list_widget.get_selected_index()?;
            let (task, _) = self.model.tasks[&selected_project].get(index)?;
            return Some(task.clone());
        }
        None
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<OverviewDialog> {
        self.selected_project?;
        let selected_project = self.selected_project.unwrap();
        let action = self.input_map.find_action(key);
        if let Some(action) = action {
            return match action {
                TasksModeActions::New => Some(self.open_new_task_dialog(selected_project)),
                TasksModeActions::Edit => self.open_edit_project_dialog(),
                TasksModeActions::Delete => self.open_delete_task_dialog(),
                TasksModeActions::AssignToProject => self.open_assign_to_project_dialog(),
            };
        }
        self.task_list_widget.handle_event(&key);
        None
    }

    fn open_new_task_dialog(&self, project_id: Uuid) -> OverviewDialog {
        let widget = TaskEditWidget::empty();
        let dialog = Dialog::new("Create new task", widget);
        OverviewDialog::CreateTask(dialog, project_id)
    }

    fn open_edit_project_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(task) = &self.get_selected_task() {
            let widget = TaskEditWidget::new(task);
            let dialog = Dialog::new("Edit the task", widget);
            return Some(OverviewDialog::EditTask(dialog, task.id));
        }
        None
    }

    fn open_delete_task_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(task) = &self.get_selected_task() {
            let widget = YesNoWidget::new();
            let dialog = Dialog::new(
                format!("Delete project \"{}\"?", task.name).as_str(),
                widget,
            );

            return Some(OverviewDialog::DeleteTask(dialog, task.id));
        }
        None
    }

    fn open_assign_to_project_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(task) = &self.get_selected_task() {
            let possible_projects_list_items = self
                .model
                .assignable_projects
                .iter()
                .map(|project| ListItem {
                    name: project.name.clone(),
                    id: project.id,
                })
                .collect::<Vec<ListItem>>();

            let widget = ListWidget::new(possible_projects_list_items);
            let dialog = Dialog::new("Assign task to project", widget);
            return Some(OverviewDialog::AssignTask(dialog, task.id));
        }
        None
    }
}
