use crate::screens::settings::settings_action::SettingsActionPurpose;
use crate::screens::settings::settings_dialog::SettingsDialog;
use ratatui::layout::Rect;
use ratatui::prelude::Modifier;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct ActionSettingsItem {
    name: String,
    description: String,
    action_target: SettingsActionPurpose,
}

impl ActionSettingsItem {
    pub fn new(name: &str, description: &str, action_target: SettingsActionPurpose) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            action_target,
        }
    }

    pub fn render(&self, frame: &mut Frame, name_area: Rect, description_area: Rect, style: Style) {
        let name_paragraph = Paragraph::new(self.name.clone()).style(style);
        frame.render_widget(name_paragraph, name_area);

        let desc_paragraph =
            Paragraph::new(self.description.clone()).style(style.add_modifier(Modifier::ITALIC));
        frame.render_widget(desc_paragraph, description_area);
    }

    pub fn execute_action(&self) -> SettingsDialog {
        self.action_target.build()
    }
}
