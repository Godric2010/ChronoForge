use crate::app_action::AppAction;
use crate::screens::settings::settings_action::SettingsActionTarget;
use domain::types::UserSettings;
use ratatui::layout::Rect;
use ratatui::prelude::Modifier;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct ToggleSettingsItem {
    name: String,
    description: String,
    value: bool,
    toggle_target: SettingsActionTarget,
}

impl ToggleSettingsItem {
    pub fn new(
        name: &str,
        description: &str,
        value: bool,
        toggle_target: SettingsActionTarget,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            value,
            toggle_target,
        }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        name_area: Rect,
        value_area: Rect,
        description_area: Rect,
        style: Style,
    ) {
        let name_paragraph = Paragraph::new(self.name.clone()).style(style);
        frame.render_widget(name_paragraph, name_area);

        let value_string = if self.value {
            "[x]".to_string()
        } else {
            "[ ]".to_string()
        };
        let value_paragraph = Paragraph::new(value_string.clone()).style(style);
        frame.render_widget(value_paragraph, value_area);

        let desc_paragraph =
            Paragraph::new(self.description.clone()).style(style.add_modifier(Modifier::ITALIC));
        frame.render_widget(desc_paragraph, description_area);
    }

    pub fn execute_action(&self) -> AppAction {
        match self.toggle_target {
            SettingsActionTarget::ShowArchivedProjects => {
                AppAction::ToggleShowArchivedProjects(!self.value)
            }
            SettingsActionTarget::ShowArchivedTasks => {
                AppAction::ToggleShowArchivedTasks(!self.value)
            }
        }
    }

    pub fn update(&mut self, settings: &UserSettings) {
        self.value = match self.toggle_target {
            SettingsActionTarget::ShowArchivedProjects => settings.show_archived_projects,
            SettingsActionTarget::ShowArchivedTasks => settings.show_archived_tasks,
        }
    }
}
