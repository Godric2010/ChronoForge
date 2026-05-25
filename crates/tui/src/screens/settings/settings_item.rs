use crate::screens::settings::settings_action::{SettingsActionPurpose, SettingsActionTarget};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[allow(dead_code)]
pub enum SettingsItemKind {
    Action(SettingsActionPurpose),
    Value(SettingsActionTarget, String),
    Toggle(SettingsActionTarget, bool),
}

pub struct SettingsItem {
    name: String,
    description: String,
    kind: SettingsItemKind,
    name_width: u16,
    value_width: u16,
}

impl SettingsItem {
    pub fn new(
        name: &str,
        description: &str,
        kind: SettingsItemKind,
        name_width: u16,
        value_width: u16,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            kind,
            name_width,
            value_width,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, selected: bool) {
        let layout = Layout::horizontal([
            Constraint::Length(self.name_width),
            Constraint::Length(self.value_width),
            Constraint::Min(self.description.len() as u16),
        ])
        .split(area);

        let style = if selected {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };

        let name_paragraph = Paragraph::new(self.name.clone()).style(style);
        frame.render_widget(name_paragraph, layout[0]);

        let value_paragraph = match &self.kind {
            SettingsItemKind::Action(_) => Paragraph::new(""),
            SettingsItemKind::Value(_, value) => Paragraph::new(value.clone()),
            SettingsItemKind::Toggle(_, toggle) => {
                let checkbox = if *toggle {
                    "[x]".to_string()
                } else {
                    "[ ]".to_string()
                };

                Paragraph::new(checkbox)
            }
        };
        frame.render_widget(
            value_paragraph.style(style.add_modifier(Modifier::BOLD)),
            layout[1],
        );

        let desc_paragraph =
            Paragraph::new(self.description.clone()).style(style.add_modifier(Modifier::ITALIC));
        frame.render_widget(desc_paragraph, layout[2]);
    }

    pub fn get_kind(&self) -> &SettingsItemKind {
        &self.kind
    }
}
