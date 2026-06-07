use crate::widgets::elements::ElementSize;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct TimeEditElement {
    hour: u32,
    minute: u32,
    max_hour: Option<u32>,
    active: bool,
    size: ElementSize,
    hour_digit_chars: Vec<char>,
    minute_digit_chars: Vec<char>,
    cursor_pos: usize,
}

impl TimeEditElement {
    pub fn new(hour: u32, minute: u32, max_hour: Option<u32>) -> Self {
        Self {
            hour,
            minute,
            max_hour,
            active: false,
            size: ElementSize {
                width: 10,
                height: 1,
            },
            hour_digit_chars: format!("{:0width$}", hour, width = 2).chars().collect(),
            minute_digit_chars: format!("{:0width$}", minute, width = 2).chars().collect(),
            cursor_pos: 0,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_hour_value(&self) -> u32 {
        self.hour
    }
    pub fn get_minute_value(&self) -> u32 {
        self.minute
    }

    pub fn get_size(&self) -> &ElementSize {
        &self.size
    }

    pub fn render(&mut self, frame: &mut Frame, pos_x: u16, pos_y: u16) {
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

    pub fn handle_key(&mut self, key: KeyEvent) {
        if !self.active {
            return;
        }

        match key.code {
            KeyCode::Up => {
                if key.modifiers == KeyModifiers::CONTROL {
                    self.increase_hour()
                } else {
                    self.increase_minute()
                }
            }
            KeyCode::Down => {
                if key.modifiers == KeyModifiers::CONTROL {
                    self.decrease_hour()
                } else {
                    self.decrease_minute()
                }
            }
            KeyCode::Left => {
                if self.cursor_pos == 0 {
                    return;
                }
                self.cursor_pos -= 1;
            }
            KeyCode::Right => {
                if self.cursor_pos
                    == self.hour_digit_chars.len() + self.minute_digit_chars.len() - 1
                {
                    return;
                }
                self.cursor_pos += 1;
            }
            KeyCode::Char(char) => {
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
            _ => {}
        }
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
