use crate::screens::settings::settings_item::{SettingsItem, SettingsItemKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct SettingsSection {
    title: String,
    items: Vec<SettingsItem>,
    height: u16,
}

impl SettingsSection {
    pub fn new(title: &str, items: Vec<SettingsItem>) -> Self {
        let height = items.len() as u16 + 1;
        Self {
            title: title.to_string(),
            items,
            height,
        }
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_items_count(&self) -> usize {
        self.items.len()
    }

    pub fn get_item_kind(&self, index: usize) -> Option<&SettingsItemKind> {
        if index < self.items.len() {
            return Some(self.items[index].get_kind());
        }
        None
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, selected_item: Option<usize>) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(self.items.len() as u16),
        ])
        .split(area);

        let title_paragraph =
            Paragraph::new(self.title.clone()).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(title_paragraph, vertical[0]);

        let horizontal =
            Layout::horizontal([Constraint::Length(1), Constraint::Min(1)]).split(vertical[1]);
        let items_rect = horizontal[1];
        for item_index in 0..self.items.len() {
            let item = &self.items[item_index];

            let item_rect = Rect {
                x: items_rect.x,
                y: items_rect.y + item_index as u16,
                width: items_rect.width,
                height: 1,
            };

            let selected = if let Some(selected_item) = selected_item {
                item_index == selected_item
            } else {
                false
            };

            item.render(frame, item_rect, selected);
        }
    }
}
