use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use uuid::Uuid;

pub struct ListItem {
    pub(crate) name: String,
    pub(crate) id: Uuid,
}
pub struct ListWidget {
    items: Vec<ListItem>,
    selected_index: usize,
    scroll_offset: usize,
    visible_count: usize,
}

impl ListWidget {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self {
            items,
            selected_index: 0,
            scroll_offset: 0,
            visible_count: 8,
        }
    }

    fn render_top_more_text(&self, frame: &mut Frame, area: Rect) {
        let more_text_top: String = if self.scroll_offset > 0 {
            "↑ more".to_string()
        } else {
            String::new()
        };
        let top_paragraph =
            Paragraph::new(more_text_top).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(top_paragraph, area);
    }
    fn render_list_items(&mut self, frame: &mut Frame, area: Rect) {
        for index in 0..self.visible_count {
            let item_index = index + self.scroll_offset;
            if item_index >= self.items.len() {
                break;
            }

            let item_area = Rect {
                x: area.x,
                y: area.y + index as u16,
                width: area.width,
                height: 1,
            };

            let paragraph = if self.selected_index == item_index {
                Paragraph::new(self.items[item_index].name.clone())
                    .style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                Paragraph::new(self.items[item_index].name.clone())
            };

            frame.render_widget(paragraph, item_area);
        }
    }
    fn render_bottom_more_text(&self, frame: &mut Frame, area: Rect) {
        let more_text_bottom: String = if self.scroll_offset + self.visible_count < self.items.len()
        {
            "↓ more".to_string()
        } else {
            String::new()
        };
        let bottom_paragraph =
            Paragraph::new(more_text_bottom).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(bottom_paragraph, area);
    }
}

impl DialogWidget for ListWidget {
    type Output = Uuid;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn get_help_text(&self) -> String {
        "<Enter>: Confirm | <Esc>: Cancel | <Up/Down>".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                self.selected_index = (self.selected_index + 1).min(self.items.len() - 1);
                if self.selected_index >= self.visible_count + self.scroll_offset {
                    self.scroll_offset += 1;
                }
            }
            KeyCode::Up => {
                if self.selected_index == 0 {
                    return;
                }
                self.selected_index -= 1;
                if self.selected_index < self.scroll_offset {
                    self.scroll_offset -= 1;
                }
            }
            _ => (),
        }
    }

    fn output(&self) -> Self::Output {
        self.items[self.selected_index].id
    }

    fn height(&self) -> u16 {
        10
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(self.visible_count as u16),
            Constraint::Length(1),
        ])
        .split(area);

        self.render_top_more_text(frame, vertical[0]);
        self.render_list_items(frame, vertical[1]);
        self.render_bottom_more_text(frame, vertical[2]);
    }
}
