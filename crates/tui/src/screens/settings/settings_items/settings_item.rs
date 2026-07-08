use crate::screens::settings::settings_action::{
    SettingsActionPurpose, TimeTargetAction, ToggleActionTarget,
};
use crate::screens::settings::settings_items::action_settings_item::ActionSettingsItem;
use crate::screens::settings::settings_items::settings_item::SettingsItem::*;
use crate::screens::settings::settings_items::time_settings_item::TimeSettingsItem;
use crate::screens::settings::settings_items::toggle_settings_item::ToggleSettingsItem;
use domain::types::UserSettings;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::Frame;

pub enum SettingsItem {
    Action(ActionSettingsItem),
    Toggle(ToggleSettingsItem),
    TimeValue(TimeSettingsItem),
}

const NAME_WIDTH: u16 = 25;
const VALUE_WIDTH: u16 = 15;
const MIN_DESCRIPTION_WIDTH: u16 = 15;

impl SettingsItem {
    pub fn new_action(name: &str, description: &str, action_target: SettingsActionPurpose) -> Self {
        Action(ActionSettingsItem::new(name, description, action_target))
    }

    pub fn new_toggle(name: &str, description: &str, toggle_target: ToggleActionTarget) -> Self {
        Toggle(ToggleSettingsItem::new(
            name,
            description,
            false,
            toggle_target,
        ))
    }

    pub fn new_time_value(name: &str, description: &str, time_target: TimeTargetAction) -> Self {
        TimeValue(TimeSettingsItem::new(name, description, 0, time_target))
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
            TimeValue(time_item) => time_item.render(frame, layout[0], layout[1], layout[2], style),
        }
    }

    pub fn update(&mut self, settings: &UserSettings) {
        match self {
            Action(_) => {}
            Toggle(toggle_item) => {
                toggle_item.update(settings);
            }
            TimeValue(time_item) => {
                time_item.update(settings);
            }
        }
    }
}
