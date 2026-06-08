use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[allow(dead_code)]
pub enum TextInputMode {
    Ascii,
    Naming,
}

pub struct TextInputWidget {
    content: String,
    mode: TextInputMode,
    cursor: usize,
}

impl TextInputWidget {
    pub fn new(mode: TextInputMode, content: Option<String>) -> Self {
        let content_str = content.unwrap_or_default();

        Self {
            mode,
            cursor: content_str.len(),
            content: content_str,
        }
    }

    fn is_char_valid(&self, c: char) -> bool {
        match self.mode {
            TextInputMode::Ascii => c.is_ascii(),
            TextInputMode::Naming => c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ',
        }
    }
}

impl DialogWidget for TextInputWidget {
    type Output = String;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn get_help_text(&self) -> String {
        "<Enter>: Confirm | <Esc>: Cancel".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace if self.cursor > 0 => {
                self.content.remove(self.cursor - 1);
                self.cursor -= 1;
            }
            KeyCode::Char(c) if self.is_char_valid(c) => {
                self.content.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Left if self.cursor > 0 => {
                self.cursor -= 1;
            }
            KeyCode::Right if self.cursor < self.content.len() => {
                self.cursor += 1;
            }
            _ => (),
        }
    }

    fn output(&self) -> Self::Output {
        self.content.clone()
    }

    fn height(&self) -> u16 {
        1
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.content.clone());
        frame.render_widget(paragraph, area);

        frame.set_cursor_position((area.x + self.cursor as u16, area.y));
    }
}
#[cfg(test)]
mod text_input_widget_tests {
    use super::*;
    use crate::widgets::test_helper::{char_key, key};
    #[test]
    fn text_input_accepts_valid_naming_chars() {
        let mut widget = TextInputWidget::new(TextInputMode::Naming, None);
        widget.handle_key(char_key('A'));
        widget.handle_key(char_key('b'));
        widget.handle_key(char_key('-'));
        widget.handle_key(char_key('_'));
        widget.handle_key(char_key(' '));
        widget.handle_key(char_key('1'));
        assert_eq!(widget.output(), "Ab-_ 1");
    }

    #[test]
    fn text_input_rejects_invalid_naming_chars() {
        let mut widget = TextInputWidget::new(TextInputMode::Naming, None);
        widget.handle_key(char_key('A'));
        widget.handle_key(char_key('/'));
        widget.handle_key(char_key('\\'));
        widget.handle_key(char_key(':'));
        widget.handle_key(char_key('*'));
        widget.handle_key(char_key('%'));
        assert_eq!(widget.output(), "A");
    }

    #[test]
    fn text_input_backspace_on_empty_string_does_not_crash() {
        let mut widget = TextInputWidget::new(TextInputMode::Naming, None);
        widget.handle_key(key(KeyCode::Backspace));
        assert_eq!(widget.output(), "");
    }

    #[test]
    fn text_input_can_insert_in_middle() {
        let mut widget = TextInputWidget::new(TextInputMode::Naming, Some("ac".to_string()));
        widget.handle_key(key(KeyCode::Left));
        widget.handle_key(char_key('b'));
        assert_eq!(widget.output(), "abc");
    }

    #[test]
    fn text_input_survives_hostile_input_sequence() {
        let mut widget = TextInputWidget::new(TextInputMode::Ascii, None);

        let keys = vec![
            key(KeyCode::Backspace),
            key(KeyCode::Left),
            key(KeyCode::Right),
            key(KeyCode::Up),
            key(KeyCode::Down),
            key(KeyCode::Home),
            char_key('a'),
            char_key('ä'),
            char_key('😊'),
            key(KeyCode::Backspace),
            key(KeyCode::Enter),
            key(KeyCode::Esc),
        ];

        for key in keys {
            widget.handle_key(key);
        }
    }
}
