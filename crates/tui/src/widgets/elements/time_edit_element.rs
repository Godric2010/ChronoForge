use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::widgets::elements::{ElementSize, WidgetElement};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

#[derive(Copy, Clone)]
enum TimeEditActions {
    IncreaseMinute,
    DecreaseMinute,
    IncreaseHour,
    DecreaseHour,
    MoveCursorForward,
    MoveCursorBackward,
}
pub struct TimeEditElement {
    hour: u32,
    minute: u32,
    max_hour: Option<u32>,
    active: bool,
    size: ElementSize,
    hour_digit_chars: Vec<char>,
    minute_digit_chars: Vec<char>,
    cursor_pos: usize,
    input_map: InputMap<TimeEditActions>,
}

impl TimeEditElement {
    pub fn new(hour: u32, minute: u32, max_hour: Option<u32>) -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Left,
                key_modifier: KeyModifiers::empty(),
                key_name: "←".to_string(),
                key_description: "Move cursor backwards".to_string(),
                action: TimeEditActions::MoveCursorBackward,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Right,
                key_modifier: KeyModifiers::empty(),
                key_name: "→".to_string(),
                key_description: "Move cursor forward".to_string(),
                action: TimeEditActions::MoveCursorForward,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Up,
                key_modifier: KeyModifiers::empty(),
                key_name: "↑".to_string(),
                key_description: "Increase Minute".to_string(),
                action: TimeEditActions::IncreaseMinute,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Down,
                key_modifier: KeyModifiers::empty(),
                key_name: "↓".to_string(),
                key_description: "Decrease Minute".to_string(),
                action: TimeEditActions::DecreaseMinute,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Up,
                key_modifier: KeyModifiers::SHIFT,
                key_name: "Shift + ↑".to_string(),
                key_description: "Increase Hour".to_string(),
                action: TimeEditActions::IncreaseHour,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Down,
                key_modifier: KeyModifiers::SHIFT,
                key_name: "Shift + ↓".to_string(),
                key_description: "Decrease Hour".to_string(),
                action: TimeEditActions::DecreaseHour,
                display_in_footer: false,
            },
        ];
        let input_map = InputMap::new("Time Edit Actions", key_bindings);

        Self {
            hour,
            minute,
            max_hour,
            active: false,
            size: ElementSize {
                width: if max_hour.is_some() { 5 } else { 10 },
                height: 1,
            },
            hour_digit_chars: format!("{:0width$}", hour, width = 2).chars().collect(),
            minute_digit_chars: format!("{:0width$}", minute, width = 2).chars().collect(),
            cursor_pos: 0,
            input_map,
        }
    }
    fn set_char_at_cursor(&mut self, char: char) {
        if !char.is_numeric() {
            return;
        }
        if self.cursor_pos >= self.hour_digit_chars.len() {
            self.edit_minute_chars(char);
        } else {
            self.edit_hour_chars(char);
        }
        self.validate_char_input();
    }
    fn move_cursor_forward(&mut self) {
        if self.cursor_pos == self.hour_digit_chars.len() + self.minute_digit_chars.len() - 1 {
            return;
        }
        self.cursor_pos += 1;
    }

    fn move_cursor_backward(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
        self.cursor_pos -= 1;
    }
    fn increase_minute(&mut self) {
        self.minute += 1;
        if self.minute > 59 {
            self.increase_hour();
            self.minute = 0;
        }
    }

    fn increase_hour(&mut self) {
        self.hour += 1;
        if let Some(max_hour) = self.max_hour {
            if self.hour >= max_hour {
                self.hour = max_hour;
            }
        }
    }

    fn decrease_minute(&mut self) {
        if self.minute == 0 {
            self.decrease_hour();
            self.minute = 59;
            return;
        }
        self.minute -= 1
    }

    fn decrease_hour(&mut self) {
        if self.hour > 0 {
            self.hour -= 1;
        }
    }

    fn edit_hour_chars(&mut self, digit: char) {
        self.hour_digit_chars[self.cursor_pos] = digit;
        self.cursor_pos += 1;
    }

    fn edit_minute_chars(&mut self, digit: char) {
        self.minute_digit_chars[self.cursor_pos - self.hour_digit_chars.len()] = digit;
        self.cursor_pos = (self.cursor_pos + 1)
            .min(self.hour_digit_chars.len() + self.minute_digit_chars.len() - 1);
    }

    fn validate_char_input(&mut self) {
        let hour_text = self
            .hour_digit_chars
            .clone()
            .into_iter()
            .collect::<String>();
        let minute_text = self
            .minute_digit_chars
            .clone()
            .into_iter()
            .collect::<String>();

        let mut hour_value = hour_text.parse::<u32>().unwrap();
        let mut minute_value = minute_text.parse::<u32>().unwrap();

        if minute_value > 59 {
            hour_value += 1;
            minute_value %= 60;
        }

        if let Some(max_hour) = self.max_hour {
            if hour_value > max_hour {
                hour_value = max_hour;
            }
        }

        self.hour = hour_value;
        self.minute = minute_value;

        self.hour_digit_chars = format!("{:0width$}", hour_value, width = 2)
            .chars()
            .collect();
        self.minute_digit_chars = format!("{:0width$}", minute_value, width = 2)
            .chars()
            .collect();
    }
}

impl HelpProvider for TimeEditElement {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
    }

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>) {
        self.input_map.append_general_help(output);
    }
}

impl WidgetElement for TimeEditElement {
    type Output = Time;

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn get_size(&self) -> &ElementSize {
        &self.size
    }

    fn render(&self, frame: &mut Frame, pos_x: u16, pos_y: u16) {
        let rect = Rect::new(pos_x, pos_y, self.size.width, self.size.height);

        let time_string = format!("{:02}:{:02}", self.hour, self.minute);
        let paragraph = Paragraph::new(time_string);
        frame.render_widget(paragraph, rect);

        if self.active {
            let offset = if self.cursor_pos > self.hour_digit_chars.len() - 1 {
                1
            } else {
                0
            };
            frame.set_cursor_position((pos_x + self.cursor_pos as u16 + offset, pos_y));
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if !self.active {
            return;
        }
        let action = self.input_map.find_action(key);
        match action {
            None => {
                if let KeyCode::Char(c) = key.code {
                    self.set_char_at_cursor(c);
                }
            }
            Some(a) => match a {
                TimeEditActions::IncreaseMinute => self.increase_minute(),
                TimeEditActions::DecreaseMinute => self.decrease_minute(),
                TimeEditActions::IncreaseHour => self.increase_hour(),
                TimeEditActions::DecreaseHour => self.decrease_hour(),
                TimeEditActions::MoveCursorForward => self.move_cursor_forward(),
                TimeEditActions::MoveCursorBackward => self.move_cursor_backward(),
            },
        }
    }

    fn get_output(&self) -> Self::Output {
        Time {
            hour: self.hour,
            minute: self.minute,
        }
    }
}
