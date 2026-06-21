use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::widgets::elements::{ElementSize, WidgetElement};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Copy, Clone)]
enum DateEditActions {
    IncreaseDay,
    DecreaseDay,
    IncreaseMonth,
    DecreaseMonth,
    MoveCursorForward,
    MoveCursorBackward,
}

pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

pub struct DateEditElement {
    day: u32,
    month: u32,
    year: u32,
    active: bool,
    size: ElementSize,
    day_digit_chars: [char; 2],
    month_digit_chars: [char; 2],
    year_digit_chars: [char; 4],
    cursor_pos: usize,
    input_map: InputMap<DateEditActions>,
}

impl DateEditElement {
    pub fn new(day: u32, month: u32, year: u32) -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Left,
                key_modifier: KeyModifiers::empty(),
                key_name: "←".to_string(),
                key_description: "Move cursor backwards".to_string(),
                action: DateEditActions::MoveCursorBackward,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Right,
                key_modifier: KeyModifiers::empty(),
                key_name: "→".to_string(),
                key_description: "Move cursor forward".to_string(),
                action: DateEditActions::MoveCursorForward,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Up,
                key_modifier: KeyModifiers::empty(),
                key_name: "↑".to_string(),
                key_description: "Increase day".to_string(),
                action: DateEditActions::IncreaseDay,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Down,
                key_modifier: KeyModifiers::empty(),
                key_name: "↓".to_string(),
                key_description: "Decrease day".to_string(),
                action: DateEditActions::DecreaseDay,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Up,
                key_modifier: KeyModifiers::SHIFT,
                key_name: "Shift + ↑".to_string(),
                key_description: "Increase month".to_string(),
                action: DateEditActions::IncreaseMonth,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Down,
                key_modifier: KeyModifiers::SHIFT,
                key_name: "Shift + ↓".to_string(),
                key_description: "Decrease month".to_string(),
                action: DateEditActions::DecreaseMonth,
                display_in_footer: false,
            },
        ];
        let input_map = InputMap::new("Date Edit Actions", key_bindings);

        Self {
            day,
            month,
            year,
            active: false,
            size: ElementSize {
                width: 10,
                height: 1,
            },
            day_digit_chars: Self::u32_to_two_chars(day),
            month_digit_chars: Self::u32_to_two_chars(month),
            year_digit_chars: Self::u32_to_four_chars(year),
            cursor_pos: 0,
            input_map,
        }
    }

    fn move_cursor_forward(&mut self) {
        if self.cursor_pos == 7 {
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

    fn set_char_at_cursor(&mut self, char: char) {
        if !char.is_numeric() {
            return;
        }
        if self.cursor_pos >= 4 {
            self.edit_year_chars(char);
        } else if self.cursor_pos >= 2 {
            self.edit_month_chars(char);
        } else {
            self.edit_day_chars(char);
        }
        self.validate_char_input();
    }

    fn increase_day(&mut self) {
        self.day += 1;
        let days_in_month = self.get_days_in_month();
        if self.day > days_in_month {
            self.increase_month();
            self.day = 1;
        }
    }

    fn increase_month(&mut self) {
        self.month += 1;
        if self.month > 12 {
            self.increase_year();
            self.month = 1;
            self.day = 1;
        }
    }

    fn increase_year(&mut self) {
        self.year += 1;
    }

    fn decrease_day(&mut self) {
        if self.day == 1 {
            self.decrease_month();
            self.day = self.get_days_in_month();
            return;
        }
        self.day -= 1;
    }

    fn decrease_month(&mut self) {
        if self.month == 1 {
            self.decrease_year();
            self.month = 12;
            self.day = 31;
            return;
        }
        self.month -= 1;
    }

    fn decrease_year(&mut self) {
        self.year -= 1;
    }

    fn get_days_in_month(&self) -> u32 {
        if self.month == 2 {
            if (self.year - 1972).is_multiple_of(4) {
                return 29;
            }
            return 28;
        }

        let months_with_31_days = [1, 3, 5, 7, 8, 10, 12];
        if months_with_31_days.contains(&self.month) {
            return 31;
        }
        30
    }

    fn edit_day_chars(&mut self, digit: char) {
        self.day_digit_chars[self.cursor_pos] = digit;
        self.cursor_pos += 1;
    }

    fn edit_month_chars(&mut self, digit: char) {
        self.month_digit_chars[self.cursor_pos - 2] = digit;
        self.cursor_pos += 1;
    }

    fn edit_year_chars(&mut self, digit: char) {
        self.year_digit_chars[self.cursor_pos - 4] = digit;
        self.cursor_pos = (self.cursor_pos + 1).min(8);
    }

    fn validate_char_input(&mut self) {
        let day_text = self.day_digit_chars.into_iter().collect::<String>();
        let month_text = self.month_digit_chars.into_iter().collect::<String>();
        let year_text = self.year_digit_chars.into_iter().collect::<String>();

        let mut day_value = day_text.parse::<u32>().unwrap();
        let mut month_value = month_text.parse::<u32>().unwrap();
        let mut year_value = year_text.parse::<u32>().unwrap();

        if year_value < 1970 {
            year_value = 1970;
        }
        self.year = year_value;
        if month_value > 12 {
            month_value = 12;
        }
        if month_value == 0 {
            month_value = 1;
        }
        self.month = month_value;
        let days_in_month = self.get_days_in_month();
        if day_value > days_in_month {
            day_value = days_in_month;
        }
        if day_value == 0 {
            day_value = 1;
        }
        self.day = day_value;

        self.day_digit_chars = Self::u32_to_two_chars(day_value);
        self.month_digit_chars = Self::u32_to_two_chars(month_value);
        self.year_digit_chars = Self::u32_to_four_chars(year_value);
    }

    fn u32_to_two_chars(value: u32) -> [char; 2] {
        let text = format!("{value:02}");
        let mut chars = ['0'; 2];
        for (index, ch) in text.chars().take(2).enumerate() {
            chars[index] = ch;
        }
        chars
    }

    fn u32_to_four_chars(value: u32) -> [char; 4] {
        let text = format!("{value:04}");
        let mut chars = ['0'; 4];
        for (index, ch) in text.chars().take(4).enumerate() {
            chars[index] = ch;
        }
        chars
    }
}

impl HelpProvider for DateEditElement {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
    }
}

impl WidgetElement for DateEditElement {
    type Output = Date;

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn get_size(&self) -> &ElementSize {
        &self.size
    }

    fn render(&self, frame: &mut Frame, pos_x: u16, pos_y: u16) {
        let rect = Rect::new(pos_x, pos_y, self.size.width, self.size.height);
        let date_string = format!("{:02}/{:02}/{:04}", self.day, self.month, self.year);
        let date_paragraph = Paragraph::new(date_string);
        frame.render_widget(date_paragraph, rect);

        if self.active {
            let offset = if self.cursor_pos > 3 {
                2
            } else if self.cursor_pos > 1 {
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
                DateEditActions::IncreaseDay => self.increase_day(),
                DateEditActions::DecreaseDay => self.decrease_day(),
                DateEditActions::IncreaseMonth => self.increase_month(),
                DateEditActions::DecreaseMonth => self.decrease_month(),
                DateEditActions::MoveCursorForward => self.move_cursor_forward(),
                DateEditActions::MoveCursorBackward => self.move_cursor_backward(),
            },
        }
    }

    fn get_output(&self) -> Self::Output {
        Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
