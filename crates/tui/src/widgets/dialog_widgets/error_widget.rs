use crate::ui_error_message::UiErrorMessage;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct ErrorWidget {
    error_message: UiErrorMessage,
    should_close: bool,
}

impl ErrorWidget {
    pub fn new(error_message: UiErrorMessage) -> Self {
        Self {
            error_message,
            should_close: false,
        }
    }
}

impl DialogWidget for ErrorWidget {
    type Output = ();

    fn get_type(&self) -> WidgetType {
        WidgetType::Error
    }

    fn get_help_text(&self) -> String {
        "<Enter>: Confirm".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                self.should_close = true;
            }
            _ => {}
        }
    }

    fn output(&self) -> Self::Output {}

    fn height(&self) -> u16 {
        4
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

        let title_paragraph = Paragraph::new(self.error_message.get_title())
            .style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(title_paragraph, vertical[0]);

        let message_paragraph = Paragraph::new(self.error_message.get_message());
        frame.render_widget(message_paragraph, vertical[1]);

        let confirm_paragraph = Paragraph::new("OK")
            .centered()
            .style(Style::default().add_modifier(Modifier::REVERSED));

        let horizontal = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(2),
            Constraint::Min(1),
        ])
        .split(vertical[3]);
        frame.render_widget(confirm_paragraph, horizontal[1]);
    }
}
