use crate::app_action::AppAction;
use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::HelpProvider;
use crate::screens::dialog::help_dialog::HelpDialog;
use crate::screens::settings::input_actions::*;
use crate::screens::settings::settings_action::{SettingsActionPurpose, SettingsActionTarget};
use crate::screens::settings::settings_dialog::{SettingsDialog, SettingsDialogResult};
use crate::screens::settings::settings_items::settings_item::SettingsItem;
use crate::screens::settings::settings_section::SettingsSection;
use crossterm::event::KeyEvent;
use domain::types::UserSettings;
use ratatui::layout::Rect;
use ratatui::Frame;

struct SelectionRef {
    section_index: usize,
    item_index: usize,
}

pub struct SettingsScreen {
    settings_data: Option<UserSettings>,
    sections: Vec<SettingsSection>,
    selection_ref: SelectionRef,
    settings_dialog: Option<SettingsDialog>,
    help_dialog: Option<HelpDialog>,
    input_map: InputMap<SettingsActions>,
    enforce_update_on_next_tick: bool,
}
impl Default for SettingsScreen {
    fn default() -> Self {
        Self::new()
    }
}
impl SettingsScreen {
    pub fn new() -> Self {
        let input_map = create_settings_input_map();

        let sections = vec![
            SettingsSection::new(
                "Import/Export",
                vec![
                    SettingsItem::new_action(
                        "Import CSV",
                        "Import Data from a CSV file into the database",
                        SettingsActionPurpose::ImportCSV,
                    ),
                    SettingsItem::new_action(
                        "Export CSV",
                        "Export Data from a CSV file of the database",
                        SettingsActionPurpose::ExportCSV,
                    ),
                ],
            ),
            SettingsSection::new(
                "Toggle show archived",
                vec![
                    SettingsItem::new_toggle(
                        "Show archived projects",
                        "Show archived projects in project overview",
                        SettingsActionTarget::ShowArchivedProjects,
                    ),
                    SettingsItem::new_toggle(
                        "Show archived tasks",
                        "Show archived tasks in task overview",
                        SettingsActionTarget::ShowArchivedTasks,
                    ),
                ],
            ),
        ];

        Self {
            settings_data: None,
            sections,
            selection_ref: SelectionRef {
                section_index: 0,
                item_index: 0,
            },
            settings_dialog: None,
            help_dialog: None,
            input_map,
            enforce_update_on_next_tick: true,
        }
    }

    pub fn update_view(&mut self, settings: UserSettings) {
        self.settings_data = Some(settings);
        if let Some(settings) = &self.settings_data {
            for section in &mut self.sections {
                section.update(settings);
            }
        }
    }

    pub fn enforce_view_update_on_next_tick(&mut self) -> bool {
        let update_on_next_tick = self.enforce_update_on_next_tick;
        self.enforce_update_on_next_tick = false;
        update_on_next_tick
    }

    pub fn get_footer_help_text(&self) -> String {
        let footer_helper = self.input_map.footer_help();
        KeyBindingHelpContext::build_single_line(footer_helper)
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
        if let Some(help_dialog) = &mut self.help_dialog {
            help_dialog.render(frame, rect);
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        if let Some(help_dialog) = self.help_dialog.as_mut() {
            let help_shall_close = help_dialog.handle_key(key_event);
            if help_shall_close {
                self.help_dialog = None;
            }
            return None;
        }

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
                SettingsActions::Select => self.select_item(),
                SettingsActions::Help => {
                    let help_context = self.input_map.general_help();
                    self.help_dialog = Some(HelpDialog::new(help_context));
                    None
                }
            }
        } else {
            None
        }
    }

    fn select_item(&mut self) -> Option<AppAction> {
        let active_item_kind = self.sections[self.selection_ref.section_index]
            .get_item(self.selection_ref.item_index)?;

        match active_item_kind {
            SettingsItem::Action(action_item) => {
                self.settings_dialog = Some(action_item.execute_action());
                None
            }
            SettingsItem::Toggle(toggle_item) => {
                self.enforce_update_on_next_tick = true;
                Some(toggle_item.execute_action())
            }
        }
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
            SettingsDialogResult::Help(help_context) => {
                self.help_dialog = Some(HelpDialog::new(help_context));
                None
            }
        }
    }
}
