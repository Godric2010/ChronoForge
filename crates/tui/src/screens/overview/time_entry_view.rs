use crate::input::input_map::InputMap;
use crate::screens::dialog::Dialog;
use crate::screens::overview::mode::TimeEntriesModeActions;
use crate::screens::overview::overview_dialog::OverviewDialog;
use crate::screens::overview::overview_input_maps::create_time_entry_mode_input_map;
use crate::screens::overview::OverviewViewModel;
use crate::widgets::dialog_widgets::{ListItem, ListWidget, TimeEntryWidget, YesNoWidget};
use crate::widgets::selectable_card_list::time_entry_card::TimeEntryCard;
use crate::widgets::selectable_card_list::SelectableCardList;
use chrono::Utc;
use crossterm::event::KeyEvent;
use domain::types::{Task, TimeEntry};
use ratatui::layout::Rect;
use ratatui::Frame;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
struct TimeEntryViewModel {
    pub assignable_tasks: Vec<Task>,
    pub time_entries: HashMap<Uuid, Vec<TimeEntry>>,
}

pub struct TimeEntryView {
    time_entry_list_widget: SelectableCardList<TimeEntryCard>,
    input_map: InputMap<TimeEntriesModeActions>,
    model: TimeEntryViewModel,
    selected_task: Option<Uuid>,
}

impl TimeEntryView {
    pub fn new() -> Self {
        let mut time_entry_list_widget = SelectableCardList::default();
        time_entry_list_widget.title = "Time Entries".to_string();

        Self {
            time_entry_list_widget,
            input_map: create_time_entry_mode_input_map(),
            model: TimeEntryViewModel::default(),
            selected_task: None,
        }
    }

    pub fn set_data_from_view_model(&mut self, view_model: &OverviewViewModel) {
        let mut model = TimeEntryViewModel::default();
        view_model.projects.iter().for_each(|p_vm| {
            let tasks = &p_vm.tasks;
            tasks.iter().for_each(|t_vm| {
                let task_id = t_vm.task.id;
                model.assignable_tasks.push(t_vm.task.clone());

                let entries_of_task = t_vm
                    .time_entries
                    .iter()
                    .map(|e_vm| e_vm.time_entry.clone())
                    .collect::<Vec<_>>();
                model.time_entries.insert(task_id, entries_of_task);
            })
        });
        self.model = model;
    }

    pub fn set_selected_task(&mut self, selected_task: Uuid) {
        if let Some(task_id) = self.selected_task {
            if task_id == selected_task {
                return;
            }
        }

        self.selected_task = Some(selected_task);
        let selected_entries = self.model.time_entries.get(&selected_task);
        if selected_entries.is_none() {
            return;
        }
        let selected_entries = selected_entries.unwrap();
        let entry_cards: Vec<TimeEntryCard> = selected_entries
            .iter()
            .map(|entry| TimeEntryCard::new(entry.start_time, entry.end_time))
            .collect::<Vec<TimeEntryCard>>();
        self.time_entry_list_widget
            .update_list_items(entry_cards, 6);
    }

    pub fn set_active(&mut self, active: bool) {
        self.time_entry_list_widget.set_active(active, active);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.time_entry_list_widget.render(frame, area);
    }

    pub fn get_selected_time_entry(&mut self) -> Option<TimeEntry> {
        if let Some(selected_task) = self.selected_task {
            let index = self.time_entry_list_widget.get_selected_index()?;
            let entry = self.model.time_entries[&selected_task].get(index)?;
            return Some(entry.clone());
        }
        None
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<OverviewDialog> {
        self.selected_task?;
        let selected_task = self.selected_task.unwrap();
        let action = self.input_map.find_action(key);
        if let Some(action) = action {
            return match action {
                TimeEntriesModeActions::New => Some(self.open_new_time_entry_dialog(selected_task)),
                TimeEntriesModeActions::Edit => self.open_edit_time_entry_dialog(),
                TimeEntriesModeActions::AssignToTask => self.open_assign_to_task_dialog(),
                TimeEntriesModeActions::Delete => self.open_delete_time_entry_dialog(),
            };
        }
        self.time_entry_list_widget.handle_event(&key);
        None
    }

    fn open_new_time_entry_dialog(&self, task_id: Uuid) -> OverviewDialog {
        let widget = TimeEntryWidget::new(Utc::now(), Utc::now());
        let dialog = Dialog::new("Create new time entry", widget);
        OverviewDialog::CreateTimeEntry(dialog, task_id)
    }

    fn open_edit_time_entry_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(time_entry) = self.get_selected_time_entry() {
            let widget = TimeEntryWidget::new(time_entry.start_time, time_entry.end_time);
            let dialog = Dialog::new("Edit time entry", widget);
            return Some(OverviewDialog::EditTimeEntry(dialog, time_entry.id));
        }
        None
    }

    fn open_delete_time_entry_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(time_entry) = self.get_selected_time_entry() {
            let widget = YesNoWidget::new();
            let dialog = Dialog::new("Delete time entry?", widget);
            return Some(OverviewDialog::DeleteTimeEntry(dialog, time_entry.id));
        }
        None
    }

    fn open_assign_to_task_dialog(&mut self) -> Option<OverviewDialog> {
        if let Some(time_entry) = self.get_selected_time_entry() {
            let possible_task_item_list = self
                .model
                .assignable_tasks
                .iter()
                .map(|task| ListItem {
                    name: task.name.clone(),
                    id: task.id,
                })
                .collect::<Vec<ListItem>>();
            let widget = ListWidget::new(possible_task_item_list);
            let dialog = Dialog::new("Assign time entry to task", widget);
            return Some(OverviewDialog::AssignTimeEntry(dialog, time_entry.id));
        }
        None
    }
}
