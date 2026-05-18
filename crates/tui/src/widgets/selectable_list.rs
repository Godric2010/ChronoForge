use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;
use ratatui::style::Color;

#[derive(Default)]
pub struct SelectableList {
    pub title: Option<String>,
    pub items: Vec<String>,
    pub highlight: bool,
    selected_index: usize,
}

impl SelectableList {
    pub fn get_selected_index(&self) -> Option<usize> {
        if self.items.len() == 0 {
            return None;
        }
        Some(self.selected_index.clone())
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let prefix = if index == self.selected_index {
                    "> "
                } else {
                    "  "
                };
                ListItem::new(Line::from(format!("{prefix}{}", item)))
            })
            .collect();

        let mut border_color = Color::Gray;
        if self.highlight {
            border_color = Color::Rgb(255, 125, 0)
        }
        let style = Style::default().fg(border_color);

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" {} ", self.title.clone().unwrap_or_default()))
                    .borders(Borders::ALL)
                    .border_style(style),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
        frame.render_widget(list, area);
    }

    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) {
        match event.code {
            KeyCode::Down => {
                self.selected_index = self.selected_index + 1;
                if self.selected_index >= self.items.len() {
                    self.selected_index = 0;
                }
            }
            KeyCode::Up => {
                if self.selected_index == 0 {
                    self.selected_index = self.items.len() - 1;
                    return;
                }
                self.selected_index = self.selected_index - 1;
            }
            _ => return,
        }
    }
}
