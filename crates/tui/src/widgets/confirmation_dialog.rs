use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;
use ratatui::style::{Color, Style};

pub enum ConfirmationResult {
    None,
    Confirmed,
    Cancelled,
}

pub struct ConfirmationDialog {
    question: String,
    width: u16,
    height: u16,
}

impl ConfirmationDialog {
    pub fn new(title: String) -> Self {
        Self {
            question: title,
            width: 45,
            height: 8,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        // outer block
        let outer_block = Block::default()
            .title("< Are you sure? >")
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Red));
        frame.render_widget(outer_block, dialog_draw_rect);

        // inner blocks
        let inner_chunks = Layout::vertical([
            Constraint::Length(1), // border
            Constraint::Length(1), // spacer
            Constraint::Length(1), // question
            Constraint::Length(1), // spacer
            Constraint::Length(1), // yes / no
        ])
        .split(dialog_draw_rect);

        // question
        let line = Line::from(self.question.clone()).alignment(HorizontalAlignment::Center);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, inner_chunks[2]);

        // yes/no
        let line = Line::from("[Y]es | [N]o").alignment(HorizontalAlignment::Center);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, inner_chunks[4]);
    }
    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let vertical_chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(self.height),
            Constraint::Min(0),
        ])
        .split(area);

        let dialog_row = vertical_chunks[1];

        let horizontal_chunks = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(self.width),
            Constraint::Min(0),
        ])
        .split(dialog_row);

        horizontal_chunks[1]
    }

    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) -> ConfirmationResult {
        match event.code {
            KeyCode::Esc => ConfirmationResult::Cancelled,
            KeyCode::Char(character) => {
                if character == 'y' {
                    return ConfirmationResult::Confirmed;
                } else if character == 'n' {
                    return ConfirmationResult::Cancelled;
                }
                ConfirmationResult::None
            }
            _ => ConfirmationResult::None,
        }
    }
}
