use crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::{symbols, Frame};
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
    scroll_offset: usize,
    visible_items_count: usize,
    width_percentage: u16,
}

impl ListDialog {
    pub fn new(title: &str, list_items: Vec<ListItem>, width_percentage: u16) -> Self {
        Self {
            title: title.to_string(),
            list_items,
            width_percentage,
            scroll_offset: 0,
            visible_items_count: 0,
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
        let inner_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner_block);

        self.visible_items_count = inner_chunks[1].height as usize;

        self.render_top_more_text(frame, inner_chunks[0]);
        self.render_list_items(frame, inner_chunks[1]);
        self.render_bottom_more_text(frame, inner_chunks[2]);

        // help box separator
        self.render_help_box(frame, inner_chunks[3], inner_chunks[4]);
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

    fn render_bottom_more_text(&self, frame: &mut Frame, area: Rect) {
        let more_text_bottom: String =
            if self.scroll_offset + self.visible_items_count < self.list_items.len() {
                "↓ more".to_string()
            } else {
                String::new()
            };
        let bottom_paragraph =
            Paragraph::new(more_text_bottom).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(bottom_paragraph, area);
    }

    fn render_help_box(&self, frame: &mut Frame, separator_area: Rect, text_area: Rect) {
        let separator =
            symbols::line::HORIZONTAL.repeat(text_area.width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = separator_area;
        rect.x = rect.x + 1;
        frame.render_widget(separator_widget, rect);

        // help box
        let help_box = Paragraph::new(
            Line::from("<Up/Down> | <Enter>: Confirm | <Esc>: Cancel").alignment(Alignment::Center),
        );
        frame.render_widget(help_box, text_area);
    }

    fn render_list_items(&mut self, frame: &mut Frame, area: Rect) {
        for index in 0..self.visible_items_count {
            let item_index = index + self.scroll_offset;
            if item_index >= self.list_items.len() {
                break;
            }

            let item_area = Rect {
                x: area.x,
                y: area.y + index as u16,
                width: area.width,
                height: 1,
            };

            let paragraph = if self.selected_index == item_index {
                Paragraph::new(self.list_items[item_index].name.clone())
                    .style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                Paragraph::new(self.list_items[item_index].name.clone())
            };

            frame.render_widget(paragraph, item_area);
        }
    }

    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let height = 18;
        let vertical_chunks = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(height),
            Constraint::Min(1),
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
                self.selected_index = (self.selected_index + 1).min(self.list_items.len() - 1);
                if self.selected_index >= self.visible_items_count + self.scroll_offset {
                    self.scroll_offset += 1;
                }
                DialogResult::None
            }
            KeyCode::Up => {
                if self.selected_index == 0 {
                    return DialogResult::None;
                }
                self.selected_index = self.selected_index - 1;
                if self.selected_index < self.scroll_offset {
                    self.scroll_offset -= 1;
                }
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
