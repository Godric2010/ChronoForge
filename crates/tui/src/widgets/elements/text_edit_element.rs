use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::widgets::elements::ElementSize;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[allow(dead_code)]
pub enum InputMode {
    Ascii,
    Naming,
}

#[derive(Copy, Clone)]
enum TextEditActions {
    RemoveCharacter,
    MoveCursorLeft,
    MoveCursorRight,
}

pub struct TextEditElement {
    content: String,
    mode: InputMode,
    cursor_pos: usize,
    size: ElementSize,
    active: bool,
    key_map: InputMap<TextEditActions>,
}

impl TextEditElement {
    pub fn new(content: Option<String>, mode: InputMode) -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Backspace,
                key_modifier: KeyModifiers::empty(),
                key_name: "Backspace".to_string(),
                key_description: "Remove the character underneath the cursor".to_string(),
                action: TextEditActions::RemoveCharacter,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Left,
                key_modifier: KeyModifiers::empty(),
                key_name: "Left".to_string(),
                key_description: "Move the cursor to the left".to_string(),
                action: TextEditActions::MoveCursorLeft,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Right,
                key_modifier: KeyModifiers::empty(),
                key_name: "Right".to_string(),
                key_description: "Move the cursor to the right".to_string(),
                action: TextEditActions::MoveCursorRight,
                display_in_footer: false,
            },
        ];
        let key_map = InputMap::new("Text Input Actions", key_bindings);

        let content = content.unwrap_or_default();
        Self {
            mode,
            cursor_pos: content.len(),
            content,
            size: ElementSize {
                width: 40,
                height: 1,
            },
            active: false,
            key_map,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_size(&self) -> &ElementSize {
        &self.size
    }

    pub fn render(&mut self, frame: &mut Frame, pos_x: u16, pos_y: u16) {
        let rect = Rect {
            x: pos_x,
            y: pos_y,
            width: self.size.width,
            height: self.size.height,
        };

        let paragraph = Paragraph::new(self.content.clone());
        frame.render_widget(paragraph, rect);

        if self.active {
            frame.set_cursor_position((pos_x + self.cursor_pos as u16, pos_y));
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if !self.active {
            return;
        }

        let action = self.key_map.find_action(key);
        if let Some(action) = action {
            match action {
                TextEditActions::RemoveCharacter => self.remove_character(),
                TextEditActions::MoveCursorLeft => self.move_cursor_left(),
                TextEditActions::MoveCursorRight => self.move_cursor_right(),
            }
            return;
        }

        if let KeyCode::Char(c) = key.code {
            self.insert_char(c);
        }
    }

    fn remove_character(&mut self) {
        if self.cursor_pos > 0 {
            self.content.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.content.len() && self.is_content_in_bounds() {
            self.cursor_pos += 1;
        }
    }

    fn insert_char(&mut self, c: char) {
        if self.is_char_valid(c) && self.is_content_in_bounds() {
            self.content.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
    }

    fn is_content_in_bounds(&self) -> bool {
        self.content.len() < self.size.width as usize
    }

    fn is_char_valid(&self, c: char) -> bool {
        match self.mode {
            InputMode::Ascii => c.is_ascii(),
            InputMode::Naming => c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ',
        }
    }
}
#[cfg(test)]
mod text_input_widget_tests {
    use super::*;
    use crate::widgets::test_helper::{char_key, key};
    #[test]
    fn text_input_accepts_valid_naming_chars() {
        let mut widget = TextEditElement::new(None, InputMode::Naming);
        widget.set_active(true);
        widget.handle_key(char_key('A'));
        widget.handle_key(char_key('b'));
        widget.handle_key(char_key('-'));
        widget.handle_key(char_key('_'));
        widget.handle_key(char_key(' '));
        widget.handle_key(char_key('1'));
        assert_eq!(widget.get_content(), "Ab-_ 1");
    }

    #[test]
    fn text_input_rejects_invalid_naming_chars() {
        let mut widget = TextEditElement::new(None, InputMode::Naming);
        widget.set_active(true);
        widget.handle_key(char_key('A'));
        widget.handle_key(char_key('/'));
        widget.handle_key(char_key('\\'));
        widget.handle_key(char_key(':'));
        widget.handle_key(char_key('*'));
        widget.handle_key(char_key('%'));
        assert_eq!(widget.get_content(), "A");
    }

    #[test]
    fn text_input_backspace_on_empty_string_does_not_crash() {
        let mut widget = TextEditElement::new(None, InputMode::Naming);
        widget.handle_key(key(KeyCode::Backspace));
        assert_eq!(widget.get_content(), "");
    }

    #[test]
    fn text_input_can_insert_in_middle() {
        let mut widget = TextEditElement::new(Some("ac".to_string()), InputMode::Naming);
        widget.set_active(true);
        widget.handle_key(key(KeyCode::Left));
        widget.handle_key(char_key('b'));
        assert_eq!(widget.get_content(), "abc");
    }

    #[test]
    fn text_input_survives_hostile_input_sequence() {
        let mut widget = TextEditElement::new(None, InputMode::Ascii);
        widget.set_active(true);

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

    #[test]
    fn text_input_exceeds_length() {
        let string = "----------------------------------------".to_string();
        let mut widget = TextEditElement::new(Some(string), InputMode::Naming);
        widget.set_active(true);
        widget.handle_key(char_key('a'));

        let result = widget.get_content();
        assert_eq!(result.len(), widget.size.width as usize);
    }

    #[test]
    fn text_input_if_not_active() {
        let mut widget = TextEditElement::new(None, InputMode::Naming);
        widget.set_active(false);
        widget.handle_key(char_key('a'));
        let result = widget.get_content();
        assert_eq!(result.len(), 0);
    }
}
