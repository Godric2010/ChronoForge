use crate::screens::settings::settings_action::{SettingsActionPurpose, SettingsActionTarget};
use crate::screens::settings::settings_items::action_settings_item::ActionSettingsItem;
use crate::screens::settings::settings_items::settings_item::SettingsItem::{Action, Toggle};
use crate::screens::settings::settings_items::toggle_settings_item::ToggleSettingsItem;
use domain::types::UserSettings;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::Frame;

pub enum SettingsItem {
    Action(ActionSettingsItem),
    Toggle(ToggleSettingsItem),
}

const NAME_WIDTH: u16 = 25;
const VALUE_WIDTH: u16 = 9;
const MIN_DESCRIPTION_WIDTH: u16 = 15;

impl SettingsItem {
    pub fn new_action(name: &str, description: &str, action_target: SettingsActionPurpose) -> Self {
        Action(ActionSettingsItem::new(name, description, action_target))
    }

    pub fn new_toggle(name: &str, description: &str, toggle_target: SettingsActionTarget) -> Self {
        Toggle(ToggleSettingsItem::new(
            name,
            description,
            false,
            toggle_target,
        ))
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, selected: bool) {
        let layout = Layout::horizontal([
            Constraint::Length(NAME_WIDTH),
            Constraint::Length(VALUE_WIDTH),
            Constraint::Min(MIN_DESCRIPTION_WIDTH),
        ])
        .split(area);

        let style = if selected {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };

        match self {
            Action(action_item) => action_item.render(frame, layout[0], layout[2], style),
            Toggle(toggle_item) => {
                toggle_item.render(frame, layout[0], layout[1], layout[2], style)
            }
        }
    }

    pub fn update(&mut self, settings: &UserSettings) {
        match self {
            Action(_) => {}
            Toggle(toggle_item) => {
                toggle_item.update(settings);
            }
        }
    }
}
