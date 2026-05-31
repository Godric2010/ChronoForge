use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Timelike, Utc};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct TimeEntryWidget {
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,

    selected_field: usize,

    start_date_error_msg: String,
    end_date_error_msg: String,

    input_fields: [DigitInputWidget; 10],
}

impl TimeEntryWidget {
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        let start_local = start_time.with_timezone(&Local);
        let end_local = end_time.with_timezone(&Local);

        let mut input_fields = [
            DigitInputWidget::new(start_local.hour() as u16, 2, 0, 23),
            DigitInputWidget::new(start_local.minute() as u16, 2, 0, 59),
            DigitInputWidget::new(start_local.day() as u16, 2, 1, 31),
            DigitInputWidget::new(start_local.month() as u16, 2, 1, 12),
            DigitInputWidget::new(start_local.year() as u16, 4, 1970, 9999),
            DigitInputWidget::new(end_local.hour() as u16, 2, 0, 23),
            DigitInputWidget::new(end_local.minute() as u16, 2, 0, 59),
            DigitInputWidget::new(end_local.day() as u16, 2, 1, 31),
            DigitInputWidget::new(end_local.month() as u16, 2, 1, 12),
            DigitInputWidget::new(end_local.year() as u16, 4, 1970, 9999),
        ];
        let selected_field = 0;
        input_fields[selected_field].set_highlight(true);

        let start_date_error_msg = String::default();
        let end_date_error_msg = String::default();

        Self {
            start_time: Some(start_time),
            end_time: Some(end_time),
            selected_field,
            start_date_error_msg,
            end_date_error_msg,
            input_fields,
        }
    }
    fn draw_heading(&self, text: &str, frame: &mut Frame, area: Rect) {
        let heading = Paragraph::new(text).style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        frame.render_widget(heading, area);
    }

    fn draw_time_edit_fields(&self, frame: &mut Frame, area: Rect, data: TimeEditFieldRenderData) {
        let chunks = Layout::horizontal([
            Constraint::Length(2), // hour
            Constraint::Length(1), // :
            Constraint::Length(2), // minutes
            Constraint::Length(1), // spacer
            Constraint::Length(2), // day
            Constraint::Length(1), // /
            Constraint::Length(2), // month
            Constraint::Length(1), // /
            Constraint::Length(4), // year
            Constraint::Length(2), // spacer
            Constraint::Min(1),    // error msg
        ])
        .split(area);

        data.hour.render(frame, chunks[0]);
        let colum = Paragraph::new(":");
        frame.render_widget(colum, chunks[1]);
        data.minute.render(frame, chunks[2]);

        data.day.render(frame, chunks[4]);
        let separator = Paragraph::new("/");
        frame.render_widget(separator, chunks[5]);
        data.month.render(frame, chunks[6]);
        let separator = Paragraph::new("/");
        frame.render_widget(separator, chunks[7]);
        data.year.render(frame, chunks[8]);

        let error_paragraph =
            Paragraph::new(data.error_msg.to_owned()).style(Style::default().fg(Color::Red));
        frame.render_widget(error_paragraph, chunks[10]);
    }
    fn validate_start_date_time(&mut self) {
        self.start_date_error_msg = String::new();
        if let Some(str) = self.input_fields[self.selected_field].get_invalid_value_error_msg() {
            self.start_date_error_msg = str;
            self.start_time = None;
            return;
        }

        let date_time = self.create_date_time_from_values(
            &self.input_fields[0],
            &self.input_fields[1],
            &self.input_fields[2],
            &self.input_fields[3],
            &self.input_fields[4],
        );

        if let Err(error) = date_time {
            self.start_date_error_msg = error.to_string();
            self.start_time = None;
            return;
        }

        let date_time = date_time.unwrap();

        let mut override_active;
        let mut start_time = None;
        if Utc::now() < date_time {
            self.start_date_error_msg = "// Time values cannot be set into the future!".to_string();
            override_active = true;
        } else {
            override_active = false;
            start_time = Some(date_time);
        }
        if let Some(end_time) = self.end_time {
            if date_time > end_time {
                self.start_date_error_msg =
                    "// End time cannot be set before start time".to_string();
                override_active = true;
                start_time = None;
            }
        }
        for idx in 0..5 {
            self.input_fields[idx].override_invalid(override_active);
        }
        self.start_time = start_time;
    }

    fn validate_end_date_time(&mut self) {
        self.end_date_error_msg = String::new();
        if let Some(str) = self.input_fields[self.selected_field].get_invalid_value_error_msg() {
            self.end_date_error_msg = str;
            self.end_time = None;
            return;
        }
        let date_time = self.create_date_time_from_values(
            &self.input_fields[5],
            &self.input_fields[6],
            &self.input_fields[7],
            &self.input_fields[8],
            &self.input_fields[9],
        );

        if let Err(error) = date_time {
            self.end_date_error_msg = error.to_string();
            self.end_time = None;
            return;
        }

        let date_time = date_time.unwrap();

        let mut override_active;
        let mut end_time = None;
        if Utc::now() < date_time {
            self.end_date_error_msg = "// Time values cannot be set into the future!".to_string();
            override_active = true;
        } else {
            override_active = false;
            end_time = Some(date_time);
        }
        if let Some(start_time) = self.start_time {
            if start_time > date_time {
                self.end_date_error_msg = "// End time cannot be set before start time".to_string();
                override_active = true;
                end_time = None;
            }
        }

        for idx in 5..10 {
            self.input_fields[idx].override_invalid(override_active);
        }
        self.end_time = end_time;
    }
    fn create_date_time_from_values(
        &self,
        hour: &DigitInputWidget,
        minute: &DigitInputWidget,
        day: &DigitInputWidget,
        month: &DigitInputWidget,
        year: &DigitInputWidget,
    ) -> anyhow::Result<DateTime<Utc>> {
        let hour_value = hour.value as u32;
        let minute_value = minute.value as u32;
        let day_value = day.value as u32;
        let month_value = month.value as u32;
        let year_value = year.value as i32;

        let date = NaiveDate::from_ymd_opt(year_value, month_value, day_value)
            .ok_or_else(|| anyhow::anyhow!("Invalid date"))?;
        let naive = date
            .and_hms_opt(hour_value, minute_value, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid time"))?;

        let local_time = Local
            .from_local_datetime(&naive)
            .single()
            .ok_or_else(|| anyhow::anyhow!("Invalid or ambiguous local time"))?;

        Ok(local_time.with_timezone(&Utc))
    }
}

impl DialogWidget for TimeEntryWidget {
    type Output = Option<(DateTime<Utc>, DateTime<Utc>)>;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn get_help_text(&self) -> String {
        "<Enter>: Confirm | <Esc>: Cancel | <Tab>: Next field | <Up>: Increase value | <Down>: Decrease value".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.input_fields[self.selected_field].set_highlight(false);
                self.selected_field = (self.selected_field + 1) % self.input_fields.len();
                self.input_fields[self.selected_field].set_highlight(true);
            }
            KeyCode::BackTab => {
                self.input_fields[self.selected_field].set_highlight(false);
                self.selected_field =
                    (self.selected_field + self.input_fields.len() - 1) % self.input_fields.len();
                self.input_fields[self.selected_field].set_highlight(true);
            }
            _ => {
                let selected_field = &mut self.input_fields[self.selected_field];
                selected_field.handle_event(&key);
                self.validate_start_date_time();
                self.validate_end_date_time();
            }
        }
    }

    fn output(&self) -> Self::Output {
        let start_time = self.start_time?;
        let end_time = self.end_time?;
        Some((start_time, end_time))
    }

    fn height(&self) -> u16 {
        7
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner_chunks = Layout::vertical([
            Constraint::Length(1), // spacer
            Constraint::Length(1), // start time heading (1)
            Constraint::Length(1), // start time fields (2)
            Constraint::Length(1), // separator (3)
            Constraint::Length(1), // end time heading (4)
            Constraint::Length(1), // end time fields (5)
            Constraint::Length(1), // spacer (6)
        ])
        .split(area);

        // start time heading
        self.draw_heading("Start timer", frame, inner_chunks[1]);
        self.draw_time_edit_fields(
            frame,
            inner_chunks[2],
            TimeEditFieldRenderData {
                hour: &self.input_fields[0],
                minute: &self.input_fields[1],
                day: &self.input_fields[2],
                month: &self.input_fields[3],
                year: &self.input_fields[4],
                error_msg: &self.start_date_error_msg,
            },
        );

        // stop time heading
        self.draw_heading("End time", frame, inner_chunks[4]);
        self.draw_time_edit_fields(
            frame,
            inner_chunks[5],
            TimeEditFieldRenderData {
                hour: &self.input_fields[5],
                minute: &self.input_fields[6],
                day: &self.input_fields[7],
                month: &self.input_fields[8],
                year: &self.input_fields[9],
                error_msg: &self.end_date_error_msg,
            },
        );
    }
}

struct DigitInputWidget {
    highlight_enabled: bool,
    is_value_invalid: bool,
    override_invalid: bool,
    value: u16,
    character_limit: u8,
    min_value: u16,
    max_value: u16,
    cursor_position: u8,
    digit_chars: Vec<char>,
}

impl DigitInputWidget {
    pub fn new(value: u16, character_limit: u8, min_value: u16, max_value: u16) -> Self {
        Self {
            highlight_enabled: false,
            is_value_invalid: false,
            override_invalid: false,
            value,
            character_limit,
            min_value,
            max_value,
            cursor_position: 0,
            digit_chars: format!("{:0width$}", value, width = character_limit as usize)
                .chars()
                .collect(),
        }
    }

    pub fn set_highlight(&mut self, enabled: bool) {
        self.highlight_enabled = enabled;
    }

    pub fn override_invalid(&mut self, enabled: bool) {
        self.override_invalid = enabled;
    }

    pub fn get_invalid_value_error_msg(&self) -> Option<String> {
        if !self.is_value_invalid {
            return None;
        }
        if self.value < self.min_value {
            return Some(format!("// Value is less than {}", self.min_value));
        }

        Some(format!("// Value is greater than {}", self.max_value))
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mut style = Style::default();
        if self.is_value_invalid || self.override_invalid {
            style = style.fg(Color::Red);
        }
        if self.highlight_enabled {
            style = style.add_modifier(Modifier::UNDERLINED);
        }

        let value_paragraph = Paragraph::new(format!(
            "{:0width$}",
            self.value,
            width = self.character_limit as usize
        ))
        .style(style);
        frame.render_widget(value_paragraph, area);

        if self.highlight_enabled {
            frame.set_cursor_position((self.cursor_position as u16 + area.x, area.y));
        }
    }

    pub fn handle_event(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Up => {
                self.value = (self.value + 1).min(self.max_value);
            }
            KeyCode::Down if self.value > self.min_value => {
                self.value -= 1;
            }
            KeyCode::Left => {
                if self.cursor_position == 0 {
                    return;
                }
                self.cursor_position -= 1;
            }
            KeyCode::Right => {
                self.cursor_position = (self.cursor_position + 1).min(self.character_limit - 1);
            }
            KeyCode::Char(char) => {
                if !char.is_numeric() {
                    return;
                }

                self.digit_chars[self.cursor_position as usize] = char;
                let new_text: String = self.digit_chars.clone().into_iter().collect();
                self.value = new_text.parse::<u16>().unwrap();

                self.is_value_invalid = self.value > self.max_value || self.value < self.min_value;

                self.cursor_position = (self.cursor_position + 1).min(self.character_limit - 1);
            }
            _ => {}
        }
    }
}
struct TimeEditFieldRenderData<'a> {
    year: &'a DigitInputWidget,
    month: &'a DigitInputWidget,
    day: &'a DigitInputWidget,
    hour: &'a DigitInputWidget,
    minute: &'a DigitInputWidget,
    error_msg: &'a str,
}

#[cfg(test)]
mod time_entry_widget_tests {
    use super::*;
    use crate::widgets::test_helper::{char_key, key};

    #[test]
    fn digit_field_down_on_zero_does_not_crash() {
        let mut digit_input_widget = DigitInputWidget::new(0, 2, 0, 99);
        digit_input_widget.handle_event(&key(KeyCode::Down));
        assert_eq!(digit_input_widget.value, 0);
    }

    #[test]
    fn digit_field_up_on_max_value_does_not_increase() {
        let mut digit_input_widget = DigitInputWidget::new(2, 1, 0, 2);
        digit_input_widget.handle_event(&key(KeyCode::Up));
        assert_eq!(digit_input_widget.value, 2);
    }

    #[test]
    fn digit_field_down_on_min_value_does_not_decrease() {
        let mut digit_input_widget = DigitInputWidget::new(1, 1, 1, 2);
        digit_input_widget.handle_event(&key(KeyCode::Down));
        assert_eq!(digit_input_widget.value, 1);
    }

    #[test]
    fn digit_field_right_moves_to_next_char_until_limit() {
        let mut digit_input_widget = DigitInputWidget::new(0, 2, 0, 99);
        digit_input_widget.handle_event(&key(KeyCode::Right));
        digit_input_widget.handle_event(&char_key('3'));
        assert_eq!(digit_input_widget.value, 3);

        digit_input_widget.handle_event(&key(KeyCode::Right));
        digit_input_widget.handle_event(&char_key('5'));
        assert_eq!(digit_input_widget.value, 5);
    }

    #[test]
    fn digit_field_left_moves_to_prev_char_until_first() {
        let mut digit_input_widget = DigitInputWidget::new(0, 2, 0, 99);
        digit_input_widget.handle_event(&key(KeyCode::Right));
        digit_input_widget.handle_event(&char_key('3'));
        assert_eq!(digit_input_widget.value, 3);

        digit_input_widget.handle_event(&key(KeyCode::Left));
        digit_input_widget.handle_event(&key(KeyCode::Left));
        digit_input_widget.handle_event(&char_key('1'));
        assert_eq!(digit_input_widget.value, 13);
    }

    #[test]
    fn digit_field_only_numerics_are_valid() {
        let mut digit_input_widget = DigitInputWidget::new(0, 4, 0, 99);
        let keys = vec![
            char_key('4'),
            char_key('a'),
            char_key('/'),
            char_key('1'),
            char_key('$'),
            char_key('2'),
            char_key(' '),
            char_key('3'),
        ];

        for key in keys {
            digit_input_widget.handle_event(&key);
        }
        assert_eq!(digit_input_widget.value, 4123);
    }

    #[test]
    fn time_entry_widget_tab_cycles_through_input_fields() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap();
        let mut widget = TimeEntryWidget::new(start, end);

        assert_eq!(widget.selected_field, 0);

        widget.handle_key(key(KeyCode::Tab));
        assert_eq!(widget.selected_field, 1);

        let tab_strokes = 9;
        for _ in 0..tab_strokes {
            widget.handle_key(key(KeyCode::Tab));
        }
        assert_eq!(widget.selected_field, 0);
    }

    #[test]
    fn time_entry_widget_back_tab_cycles_backwards_through_input_fields() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap();
        let mut widget = TimeEntryWidget::new(start, end);
        assert_eq!(widget.selected_field, 0);

        widget.handle_key(key(KeyCode::BackTab));
        assert_eq!(widget.selected_field, 9);
    }

    #[test]
    fn time_entry_widget_invalid_day_returns_none() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap();
        let mut widget = TimeEntryWidget::new(start, end);

        widget.handle_key(key(KeyCode::Tab));
        widget.handle_key(key(KeyCode::Tab));
        widget.handle_key(char_key('3'));
        widget.handle_key(char_key('4'));

        assert!(widget.output().is_none());

        widget.handle_key(char_key('1'));
        assert!(widget.output().is_some());
    }

    #[test]
    fn time_entry_widget_start_time_before_end_time_returns_none() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 11, 0, 0).unwrap();

        let mut widget = TimeEntryWidget::new(start, end);

        for _ in 0..5 {
            widget.handle_key(key(KeyCode::Tab));
        }

        widget.handle_key(key(KeyCode::Down));
        widget.handle_key(key(KeyCode::Down));
        assert!(widget.output().is_none());

        widget.handle_key(key(KeyCode::Up));
        widget.handle_key(key(KeyCode::Up));
        assert!(widget.output().is_some());

        for _ in 0..5 {
            widget.handle_key(key(KeyCode::BackTab));
        }

        widget.handle_key(key(KeyCode::Up));
        widget.handle_key(key(KeyCode::Up));
        assert!(widget.output().is_none());
    }
}
