use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;
use uuid::Uuid;

pub enum DialogResult {
    None,
    Confirmed(Uuid),
    Cancelled,
}

pub struct ListItem {
    pub name: String,
    pub id: Uuid,
}
pub struct ListDialog {
    title: String,
    list_items: Vec<ListItem>,
    selected_index: usize,
    width_percentage: u16,
}

impl ListDialog {
    pub fn new(title: &str, list_items: Vec<ListItem>, width_percentage: u16) -> Self {
        Self {
            title: title.to_string(),
            list_items,
            width_percentage,
            selected_index: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        // outer block
        let outer_block = Block::default()
            .title(format!("< {} >", self.title.clone()))
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner_block = outer_block.inner(dialog_draw_rect);
        frame.render_widget(outer_block, dialog_draw_rect);

        // inner blocks
        for index in 0..self.list_items.len() {
            let item_area = Rect {
                x: inner_block.x,
                y: inner_block.y + index as u16,
                width: inner_block.width,
                height: 1,
            };

            let paragraph = if self.selected_index == index {
                Paragraph::new(self.list_items[index].name.clone())
                    .style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                Paragraph::new(self.list_items[index].name.clone())
            };

            frame.render_widget(paragraph, item_area);
        }
    }

    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let height = self.list_items.len() as u16 + 4;
        let height_percentage = height.min(area.height);
        let vertical_chunks = Layout::vertical([
            Constraint::Percentage((100 - height_percentage) / 2),
            Constraint::Length(height),
            Constraint::Percentage((100 - height_percentage) / 2),
        ])
        .split(area);

        let dialog_row = vertical_chunks[1];

        let horizontal_chunks = Layout::horizontal([
            Constraint::Percentage((100 - self.width_percentage) / 2),
            Constraint::Percentage(self.width_percentage),
            Constraint::Percentage((100 - self.width_percentage) / 2),
        ])
        .split(dialog_row);

        horizontal_chunks[1]
    }

    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) -> DialogResult {
        match event.code {
            KeyCode::Down => {
                self.selected_index = (self.selected_index + 1) % self.list_items.len();
                DialogResult::None
            }
            KeyCode::Up => {
                if self.selected_index == 0 {
                    self.selected_index = self.list_items.len() - 1;
                    return DialogResult::None;
                }
                self.selected_index = self.selected_index - 1;
                DialogResult::None
            }
            KeyCode::Enter => {
                DialogResult::Confirmed(self.list_items[self.selected_index].id.clone())
            }
            KeyCode::Esc => DialogResult::Cancelled,
            _ => DialogResult::None,
        }
    }
}
