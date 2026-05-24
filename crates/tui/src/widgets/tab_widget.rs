use crate::screens::ScreenType;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

struct TabItem {
    name: String,
    key_value: KeyCode,
    width: usize,
    active: bool,
    screen: ScreenType,
}

impl TabItem {
    pub fn new(name: String, key_value: KeyCode, screen: ScreenType) -> Self {
        let width = name.len() + key_value.to_string().len() + 2;
        Self {
            name,
            key_value,
            width,
            screen,
            active: false,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let content = format!("[{}] {}", self.key_value, self.name);

        let paragraph = if self.active {
            Paragraph::new(content).style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Rgb(255, 125, 0)),
            )
        } else {
            Paragraph::new(content)
        };

        frame.render_widget(paragraph, area);
    }
}

pub struct TabWidget {
    items: Vec<TabItem>,
    active_index: usize,
}

impl TabWidget {
    pub fn new() -> Self {
        Self {
            items: vec![],
            active_index: 0,
        }
    }

    pub fn add_item(mut self, item_name: &str, item_key: KeyCode, screen: ScreenType) -> Self {
        self.items
            .push(TabItem::new(item_name.to_string(), item_key, screen));
        self
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut width_offset = 0;
        for index in 0..self.items.len() {
            let item = &mut self.items[index];
            let width = item.get_width() as u16 + 4;
            let rect = Rect {
                x: area.x + width_offset,
                y: area.y,
                width,
                height: area.height,
            };
            width_offset += width;

            let border_color;
            if index == self.active_index {
                item.set_active(true);
                border_color = Color::Rgb(255, 125, 0);
            } else {
                item.set_active(false);
                border_color = Color::Gray;
            }

            let tab_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(border_color));
            let inner_tab_block = tab_block.inner(rect);
            frame.render_widget(tab_block, rect);

            item.render(frame, inner_tab_block);
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<ScreenType> {
        for index in 0..self.items.len() {
            let item = &mut self.items[index];
            if item.key_value == key_event.code {
                self.active_index = index;
                return Some(item.screen);
            }
        }
        None
    }
}
