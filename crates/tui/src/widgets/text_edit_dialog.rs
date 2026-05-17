use crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::{symbols, Frame};

pub enum DialogResult {
    None,
    Confirmed(String),
    Cancelled,
}

pub struct TextEditDialog {
    title: String,
    content: String,
    cursor_pos: u16,
    cursor_start_pos: u16,
    width_percentage: u16,
}

impl TextEditDialog {
    pub fn new(title: &str, content: String, width_percentage: u16) -> Self {
        Self {
            title: title.to_string(),
            cursor_pos: content.len() as u16,
            cursor_start_pos: 0,
            content,
            width_percentage,
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
        frame.render_widget(outer_block, dialog_draw_rect);

        // inner blocks
        let inner_chunks = Layout::vertical([
            Constraint::Length(1), // top border
            Constraint::Length(1), // spacer
            Constraint::Length(1), // text box (2)
            Constraint::Length(1), // spacer
            Constraint::Length(1), // separator (4)
            Constraint::Length(1), // help text (5)
        ])
        .split(dialog_draw_rect);

        // text box
        let text_box = Paragraph::new(Line::from(self.content.clone()));
        let mut text_box_rect = inner_chunks[2];
        text_box_rect.x += 2;
        frame.render_widget(text_box, text_box_rect);

        // separator
        let separator =
            symbols::line::HORIZONTAL.repeat(inner_chunks[4].width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = inner_chunks[4];
        rect.x = rect.x + 1;
        frame.render_widget(separator_widget, rect);

        // help box
        let help_box = Paragraph::new(
            Line::from("<Enter>: Confirm | <Esc>: Cancel").alignment(Alignment::Center),
        );
        frame.render_widget(help_box, inner_chunks[5]);

        // set cursor
        self.cursor_start_pos = inner_chunks[1].x + 2;
        frame.set_cursor_position((
            self.cursor_start_pos + self.cursor_pos,
            inner_chunks[1].y + 1,
        ));
    }

    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let height = 7;
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
            KeyCode::Esc => {
                return DialogResult::Cancelled;
            }
            KeyCode::Enter => {
                return DialogResult::Confirmed(self.content.clone());
            }
            KeyCode::Backspace => {
                self.content.pop();
                self.cursor_pos = self.content.len() as u16;
            }
            KeyCode::Char(c) => {
                if c.is_alphanumeric() {
                    self.content += &c.to_string();
                    self.cursor_pos += 1;
                    return DialogResult::None
                }
                if c == ' '{
                    self.content.push(' ');
                    self.cursor_pos += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < self.content.len() as u16 {
                    self.cursor_pos += 1;
                }
            }
            _ => return DialogResult::None,
        }
        DialogResult::None
    }
}
