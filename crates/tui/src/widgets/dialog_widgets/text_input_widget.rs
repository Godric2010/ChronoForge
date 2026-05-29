use crate::widgets::dialog_widgets::DialogWidget;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub enum TextInputMode {
    AllowAll,
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
            TextInputMode::AllowAll => true,
            TextInputMode::Naming => c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ',
        }
    }
}

impl DialogWidget for TextInputWidget {
    type Output = String;

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
