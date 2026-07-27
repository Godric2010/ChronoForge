use crate::screens::settings::settings_items::settings_item::SettingsItem;
use domain::types::UserSettings;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct SettingsSection {
    title: String,
    subtitle: Option<String>,
    items: Vec<SettingsItem>,
}

impl SettingsSection {
    pub fn new(title: &str, items: Vec<SettingsItem>) -> Self {
        Self {
            title: title.to_string(),
            subtitle: None,
            items,
        }
    }

    pub fn get_height(&self) -> u16 {
        let items_count = self.items.len() as u16;
        let title_size = 1u16;
        let subtitle_size = if self.subtitle.is_some() { 1 } else { 0 } as u16;
        title_size + subtitle_size + items_count
    }

    pub fn get_items_count(&self) -> usize {
        self.items.len()
    }

    pub fn set_subtitle(&mut self, subtitle: Option<String>) {
        self.subtitle = subtitle;
    }

    pub fn get_item(&self, index: usize) -> Option<&SettingsItem> {
        if index < self.items.len() {
            let item = &self.items[index];
            return Some(item);
        }
        None
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, selected_item: Option<usize>) {
        let subtitle_size = if self.subtitle.is_some() { 1 } else { 0 } as u16;
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(subtitle_size),
            Constraint::Length(self.items.len() as u16),
        ])
        .split(area);

        let title_paragraph =
            Paragraph::new(self.title.clone()).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(title_paragraph, vertical[0]);

        if let Some(subtitle) = &self.subtitle {
            let subtitle_paragraph = Paragraph::new(subtitle.clone())
                .style(Style::default().add_modifier(Modifier::ITALIC));
            frame.render_widget(subtitle_paragraph, vertical[1]);
        }

        let horizontal =
            Layout::horizontal([Constraint::Length(1), Constraint::Min(1)]).split(vertical[2]);
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

    pub fn update(&mut self, settings: &UserSettings) {
        for item in &mut self.items {
            item.update(settings);
        }
    }
}
