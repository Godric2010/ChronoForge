use crate::app_action::AppAction;
use crate::input::input_map::InputMap;
use crate::screens::settings::input_actions::*;
use crate::screens::settings::settings_action::SettingsActionPurpose;
use crate::screens::settings::settings_dialog::{SettingsDialog, SettingsDialogResult};
use crate::screens::settings::settings_item::{SettingsItem, SettingsItemKind};
use crate::screens::settings::settings_section::SettingsSection;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::Frame;

struct SelectionRef {
    section_index: usize,
    item_index: usize,
}

pub struct SettingsScreen {
    help_text: String,
    sections: Vec<SettingsSection>,
    selection_ref: SelectionRef,
    settings_dialog: Option<SettingsDialog>,
    input_map: InputMap<SettingsActions>,
}
impl Default for SettingsScreen {
    fn default() -> Self {
        Self::new()
    }
}
impl SettingsScreen {
    pub fn new() -> Self {
        let item_name_width: u16 = 15;
        let item_value_width: u16 = 9;
        let input_map = create_settings_input_map();

        let sections = vec![SettingsSection::new(
            "Import/Export",
            vec![
                SettingsItem::new(
                    "Import CSV",
                    "Import Data from a CSV file into the database",
                    SettingsItemKind::Action(SettingsActionPurpose::ImportCSV),
                    item_name_width,
                    item_value_width,
                ),
                SettingsItem::new(
                    "Export CSV",
                    "Export Data from a CSV file of the database",
                    SettingsItemKind::Action(SettingsActionPurpose::ExportCSV),
                    item_name_width,
                    item_value_width,
                ),
            ],
        )];

        Self {
            help_text: "<Down>: Next | <Up>: Prev | <Tab>: Next section | <Shift + Tab>: Prev section | <Enter>: Confirm | <Esc/q>: Quit".to_string(),
            sections,
            selection_ref: SelectionRef {
                section_index: 0,
                item_index: 0,
            },
            settings_dialog: None,
            input_map
        }
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }

    pub fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let mut height_offset = 0;
        for section_index in 0..self.sections.len() {
            let item = &self.sections[section_index];
            let mut item_rect = rect;
            item_rect.y += height_offset;

            height_offset += item.get_height() + 1;

            let selected_item = if self.selection_ref.section_index == section_index {
                Some(self.selection_ref.item_index)
            } else {
                None
            };

            item.render(frame, item_rect, selected_item);
        }

        if let Some(dialog) = &mut self.settings_dialog {
            dialog.render(frame, rect);
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(dialog) = &mut self.settings_dialog {
            let dialog_result = dialog.handle_input(key_event);
            return self.interpret_dialog_result(dialog_result);
        }

        let action = self.input_map.find_action(key_event);
        if let Some(action) = action {
            match action {
                SettingsActions::Quit => Some(AppAction::Quit),
                SettingsActions::NextItem => {
                    self.select_next_item();
                    None
                }
                SettingsActions::PrevItem => {
                    self.select_prev_item();
                    None
                }
                SettingsActions::NextSection => {
                    self.select_next_section();
                    None
                }
                SettingsActions::PrevSection => {
                    self.select_prev_section();
                    None
                }
                SettingsActions::Select => {
                    self.select_item();
                    None
                }
            }
        } else {
            None
        }
    }

    fn select_item(&mut self) -> Option<AppAction> {
        let active_item_kind = self.sections[self.selection_ref.section_index]
            .get_item_kind(self.selection_ref.item_index)?;
        self.settings_dialog = match active_item_kind {
            SettingsItemKind::Action(purpose) => Some(purpose.build()),
            SettingsItemKind::Value(_, _) => return None,
            SettingsItemKind::Toggle(_, _) => return None,
        };
        None
    }

    fn select_prev_section(&mut self) -> Option<AppAction> {
        let section_index = self.selection_ref.section_index;
        if section_index > 0 {
            self.selection_ref.section_index -= 1;
            self.selection_ref.item_index = 0;
        }
        None
    }

    fn select_next_section(&mut self) -> Option<AppAction> {
        let section_index = self.selection_ref.section_index;
        if section_index < self.sections.len() - 1 {
            self.selection_ref.item_index = 0;
            self.selection_ref.section_index += 1;
        }
        None
    }

    fn select_prev_item(&mut self) -> Option<AppAction> {
        let item_index = self.selection_ref.item_index;
        let section_index = self.selection_ref.section_index;

        if item_index > 0 {
            self.selection_ref.item_index -= 1;
        } else if section_index > 0 {
            self.selection_ref.section_index -= 1;
            self.selection_ref.item_index = self.sections[section_index].get_items_count() - 1;
        }
        None
    }

    fn select_next_item(&mut self) -> Option<AppAction> {
        let item_index = self.selection_ref.item_index;
        let section_index = self.selection_ref.section_index;

        if item_index < self.sections[section_index].get_items_count() - 1 {
            self.selection_ref.item_index += 1;
        } else if section_index < self.sections.len() - 1 {
            self.selection_ref.section_index += 1;
            self.selection_ref.item_index = 0;
        }
        None
    }

    fn interpret_dialog_result(
        &mut self,
        dialog_result: SettingsDialogResult,
    ) -> Option<AppAction> {
        match dialog_result {
            SettingsDialogResult::None => None,
            SettingsDialogResult::Cancelled => {
                self.settings_dialog = None;
                None
            }
            SettingsDialogResult::Confirmed(app_action) => {
                self.settings_dialog = None;
                Some(app_action)
            }
        }
    }
}
