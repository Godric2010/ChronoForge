use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub enum EditResult {
    None,
    Confirmed(DateTime<Utc>, DateTime<Utc>),
    Cancelled,
}

pub struct TimeEditDialog {
    title: String,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    selected_field: usize,
    width_percentage: u16,
    start_date_error_msg: String,
    end_date_error_msg: String,
    input_fields: [NumberInputField; 10],
}

impl TimeEditDialog {
    pub fn new(
        title: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        width_percentage: u16,
    ) -> Self {
        let mut input_fields = [
            NumberInputField::new(0, start.hour() as u16, 2, 0, 23),
            NumberInputField::new(1, start.minute() as u16, 2, 0, 59),
            NumberInputField::new(2, start.day() as u16, 2, 1, 31),
            NumberInputField::new(3, start.month() as u16, 2, 1, 12),
            NumberInputField::new(4, start.year() as u16, 4, 1970, 9999),
            NumberInputField::new(5, end.hour() as u16, 2, 0, 23),
            NumberInputField::new(6, end.minute() as u16, 2, 0, 59),
            NumberInputField::new(7, end.day() as u16, 2, 1, 31),
            NumberInputField::new(8, end.month() as u16, 2, 1, 12),
            NumberInputField::new(9, end.year() as u16, 4, 1970, 9999),
        ];
        let selected_field = 0;
        input_fields[selected_field].set_highlight(true);

        let start_date_error_msg = String::default();
        let end_date_error_msg = String::default();

        Self {
            title: title.to_string(),
            start_time: Some(start),
            end_time: Some(end),
            selected_field,
            width_percentage,
            start_date_error_msg,
            end_date_error_msg,
            input_fields,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        // outer block
        let outer_block = Block::default()
            .title(format!("< {} >", self.title.clone()))
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner_block = outer_block.inner(dialog_draw_rect);
        frame.render_widget(outer_block, dialog_draw_rect);

        // inner blocks
        let inner_chunks = Layout::vertical([
            Constraint::Length(1), // spacer
            Constraint::Length(1), // start time heading (1)
            Constraint::Length(1), // start time fields (2)
            Constraint::Length(1), // separator (3)
            Constraint::Length(1), // end time heading (4)
            Constraint::Length(1), // end time fields (5)
            Constraint::Length(1), // spacer (6)
        ])
        .split(inner_block);

        // start time heading
        self.draw_heading("Start timer", frame, inner_chunks[1]);
        self.draw_time_edit_fields(
            frame,
            inner_chunks[2],
            &self.input_fields[0],
            &self.input_fields[1],
            &self.input_fields[2],
            &self.input_fields[3],
            &self.input_fields[4],
            &self.start_date_error_msg,
        );

        // stop time heading
        self.draw_heading("End time", frame, inner_chunks[4]);
        self.draw_time_edit_fields(
            frame,
            inner_chunks[5],
            &self.input_fields[5],
            &self.input_fields[6],
            &self.input_fields[7],
            &self.input_fields[8],
            &self.input_fields[9],
            &self.end_date_error_msg,
        );
    }
    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let height = 11;
        let height_percentage = height.min(area.height);
        let vertical_chunks = Layout::vertical([
            Constraint::Percentage((100 - height_percentage) / 2),
            Constraint::Length(height),
            Constraint::Percentage((100 - height_percentage) / 2),
        ])
        .split(area);

        let dialog_row = vertical_chunks[1];

        let horizontal_chunks = Layout::horizontal([
            Constraint::Percentage((100 - self.width_percentage) / 2),
            Constraint::Percentage(self.width_percentage),
            Constraint::Percentage((100 - self.width_percentage) / 2),
        ])
        .split(dialog_row);

        horizontal_chunks[1]
    }

    fn draw_heading(&self, text: &str, frame: &mut Frame, area: Rect) {
        let heading = Paragraph::new(text).style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        frame.render_widget(heading, area);
    }

    fn draw_time_edit_fields(
        &self,
        frame: &mut Frame,
        area: Rect,
        hour: &NumberInputField,
        minute: &NumberInputField,
        day: &NumberInputField,
        month: &NumberInputField,
        year: &NumberInputField,
        error_msg: &String,
    ) {
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

        hour.render(frame, chunks[0]);
        let colum = Paragraph::new(":");
        frame.render_widget(colum, chunks[1]);
        minute.render(frame, chunks[2]);

        day.render(frame, chunks[4]);
        let separator = Paragraph::new("/");
        frame.render_widget(separator, chunks[5]);
        month.render(frame, chunks[6]);
        let separator = Paragraph::new("/");
        frame.render_widget(separator, chunks[7]);
        year.render(frame, chunks[8]);

        let error_paragraph =
            Paragraph::new(error_msg.clone()).style(Style::default().fg(Color::Red));
        frame.render_widget(error_paragraph, chunks[10]);
    }

    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) -> EditResult {
        match event.code {
            KeyCode::Esc => EditResult::Cancelled,
            KeyCode::Enter => {
                let start_time = self.start_time;
                if start_time.is_none() {
                    return EditResult::None;
                }

                let end_time = self.end_time;
                if end_time.is_none() {
                    return EditResult::None;
                }

                EditResult::Confirmed(start_time.unwrap(), end_time.unwrap())
            }
            KeyCode::Tab => {
                self.input_fields[self.selected_field].set_highlight(false);
                self.selected_field = (self.selected_field + 1) % self.input_fields.len();
                self.input_fields[self.selected_field].set_highlight(true);
                EditResult::None
            }
            _ => {
                let selected_field = &mut self.input_fields[self.selected_field];
                selected_field.handle_event(event);

                if self.selected_field < 5 {
                    self.validate_start_date_time()
                } else {
                    self.validate_end_date_time()
                }

                EditResult::None
            }
        }
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

        let override_active;
        let mut start_time = None;
        if Utc::now() < date_time {
            self.start_date_error_msg = "// Time values cannot be set into the future!".to_string();
            override_active = true;
        } else {
            override_active = false;
            start_time = Some(date_time);
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

        let mut override_active = false;
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
        hour: &NumberInputField,
        minute: &NumberInputField,
        day: &NumberInputField,
        month: &NumberInputField,
        year: &NumberInputField,
    ) -> DateTime<Utc> {
        let hour_value = hour.value as u32;
        let minute_value = minute.value as u32;
        let day_value = day.value as u32;
        let month_value = month.value as u32;
        let year_value = year.value as i32;
        let date_time = Utc.with_ymd_and_hms(
            year_value,
            month_value,
            day_value,
            hour_value,
            minute_value,
            0,
        );

        date_time.unwrap()
    }
}

struct NumberInputField {
    pub id: u8,
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

impl NumberInputField {
    pub fn new(id: u8, value: u16, character_limit: u8, min_value: u16, max_value: u16) -> Self {
        Self {
            id,
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

    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) {
        match event.code {
            KeyCode::Up => {
                self.value = (self.value + 1).min(self.max_value);
            }
            KeyCode::Down => {
                self.value = (self.value - 1).max(self.min_value);
            }
            KeyCode::Left => {
                if self.cursor_position == 0 {
                    return;
                }
                self.cursor_position = (self.cursor_position - 1).max(0);
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

                if self.value > self.max_value || self.value < self.min_value {
                    self.is_value_invalid = true;
                } else {
                    self.is_value_invalid = false;
                }

                self.cursor_position = (self.cursor_position + 1).min(self.character_limit - 1);
            }
            _ => {}
        }
    }
}
