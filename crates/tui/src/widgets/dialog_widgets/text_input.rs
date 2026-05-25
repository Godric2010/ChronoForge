use crate::widgets::dialog_widgets::DialogWidget;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct TextInputWidget {
    content: String,
    cursor: usize,
}

impl TextInputWidget {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor: 0,
        }
    }
}

impl DialogWidget for TextInputWidget {
    type Output = String;

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.content.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            KeyCode::Char(c) => {
                self.content.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor < self.content.len() {
                    self.cursor += 1;
                }
            }
            _ => return,
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
