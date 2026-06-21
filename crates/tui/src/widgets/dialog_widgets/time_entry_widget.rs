use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crate::widgets::elements::{DateEditElement, TimeEditElement, WidgetElement};
use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Timelike, Utc};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Clone, Copy)]
enum TimeEntryActions {
    Next,
    Previous,
}

pub struct TimeEntryWidget {
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,

    selected_field: usize,

    start_date_error_msg: String,
    end_date_error_msg: String,

    start_time_edit_element: TimeEditElement,
    end_time_edit_element: TimeEditElement,
    start_date_edit_element: DateEditElement,
    end_date_edit_element: DateEditElement,

    input_map: InputMap<TimeEntryActions>,
}

impl TimeEntryWidget {
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        let start_local = start_time.with_timezone(&Local);
        let end_local = end_time.with_timezone(&Local);

        let mut start_time_edit_element =
            TimeEditElement::new(start_local.hour(), start_local.minute(), Some(24));
        let end_time_edit_element =
            TimeEditElement::new(end_local.hour(), end_local.minute(), Some(24));
        let start_date_edit_element = DateEditElement::new(
            start_local.day(),
            start_local.month(),
            start_local.year() as u32,
        );
        let end_date_edit_element =
            DateEditElement::new(end_local.day(), end_local.month(), end_local.year() as u32);

        let selected_field = 0;
        start_time_edit_element.set_active(true);

        let start_date_error_msg = String::default();
        let end_date_error_msg = String::default();

        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Tab,
                key_modifier: KeyModifiers::empty(),
                key_name: "⇄".to_string(),
                key_description: "Next input field".to_string(),
                action: TimeEntryActions::Next,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::BackTab,
                key_modifier: KeyModifiers::empty(),
                key_name: "BackTab".to_string(),
                key_description: "Prev input field".to_string(),
                action: TimeEntryActions::Previous,
                display_in_footer: false,
            },
        ];
        let input_map = InputMap::new("Time Entry Actions", key_bindings);

        Self {
            start_time: Some(start_time),
            end_time: Some(end_time),
            selected_field,
            start_date_error_msg,
            end_date_error_msg,
            start_time_edit_element,
            start_date_edit_element,
            end_time_edit_element,
            end_date_edit_element,
            input_map,
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

    fn draw_time_edit_fields(
        &self,
        frame: &mut Frame,
        area: Rect,
        date_field: &DateEditElement,
        time_field: &TimeEditElement,
        error_msg: &str,
    ) {
        let chunks = Layout::horizontal([
            Constraint::Length(time_field.get_size().width), // time
            Constraint::Length(1),                           // spacer
            Constraint::Length(date_field.get_size().width), // date
            Constraint::Length(2),                           // spacer
            Constraint::Min(1),                              // error msg
        ])
        .split(area);

        time_field.render(frame, chunks[0].x, chunks[0].y);
        date_field.render(frame, chunks[2].x, chunks[2].y);

        let error_paragraph = Paragraph::new(error_msg).style(Style::default().fg(Color::Red));
        frame.render_widget(error_paragraph, chunks[4]);
    }
    fn validate_start_date_time(&mut self) {
        self.start_date_error_msg = String::new();

        let date_time = self.create_date_time_from_values(
            &self.start_date_edit_element,
            &self.start_time_edit_element,
        );

        if let Err(error) = date_time {
            self.start_date_error_msg = error.to_string();
            self.start_time = None;
            return;
        }

        let date_time = date_time.unwrap();

        let mut start_time = None;
        if Utc::now() < date_time {
            self.start_date_error_msg = "// Time values cannot be set into the future!".to_string();
        } else {
            start_time = Some(date_time);
        }
        if let Some(end_time) = self.end_time {
            if date_time > end_time {
                self.start_date_error_msg =
                    "// End time cannot be set before start time".to_string();
                start_time = None;
            }
        }
        self.start_time = start_time;
    }

    fn validate_end_date_time(&mut self) {
        self.end_date_error_msg = String::new();
        let date_time = self
            .create_date_time_from_values(&self.end_date_edit_element, &self.end_time_edit_element);

        if let Err(error) = date_time {
            self.end_date_error_msg = error.to_string();
            self.end_time = None;
            return;
        }

        let date_time = date_time.unwrap();

        let mut end_time = None;
        if Utc::now() < date_time {
            self.end_date_error_msg = "// Time values cannot be set into the future!".to_string();
        } else {
            end_time = Some(date_time);
        }
        if let Some(start_time) = self.start_time {
            if start_time > date_time {
                self.end_date_error_msg = "// End time cannot be set before start time".to_string();
                end_time = None;
            }
        }

        self.end_time = end_time;
    }
    fn create_date_time_from_values(
        &self,
        date: &DateEditElement,
        time: &TimeEditElement,
    ) -> anyhow::Result<DateTime<Utc>> {
        let date_value = date.get_output();
        let time_value = time.get_output();

        let date =
            NaiveDate::from_ymd_opt(date_value.year as i32, date_value.month, date_value.day)
                .ok_or_else(|| anyhow::anyhow!("Invalid date"))?;
        let naive = date
            .and_hms_opt(time_value.hour, time_value.minute, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid time"))?;

        let local_time = Local
            .from_local_datetime(&naive)
            .single()
            .ok_or_else(|| anyhow::anyhow!("Invalid or ambiguous local time"))?;

        Ok(local_time.with_timezone(&Utc))
    }

    fn toggle_selected_field(&mut self, enabled: bool) {
        match self.selected_field {
            0 => self.start_time_edit_element.set_active(enabled),
            1 => self.start_date_edit_element.set_active(enabled),
            2 => self.end_time_edit_element.set_active(enabled),
            3 => self.end_date_edit_element.set_active(enabled),
            _ => {}
        }
    }

    fn handle_input_for_active_element(&mut self, key: KeyEvent) {
        match self.selected_field {
            0 => self.start_time_edit_element.handle_key(key),
            1 => self.start_date_edit_element.handle_key(key),
            2 => self.end_time_edit_element.handle_key(key),
            3 => self.end_date_edit_element.handle_key(key),
            _ => {}
        }
    }

    fn select_next_field(&mut self) {
        self.toggle_selected_field(false);
        self.selected_field = (self.selected_field + 1) % 4;
        self.toggle_selected_field(true);
    }

    fn select_previous_field(&mut self) {
        self.toggle_selected_field(false);
        self.selected_field = (self.selected_field + 4 - 1) % 4;
        self.toggle_selected_field(true);
    }

    fn handle_input_field_input(&mut self, key: KeyEvent) {
        self.handle_input_for_active_element(key);
        self.validate_start_date_time();
        self.validate_end_date_time();
    }
}
impl HelpProvider for TimeEntryWidget {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
        match self.selected_field {
            0 => self.start_time_edit_element.append_footer_help(output),
            1 => self.start_date_edit_element.append_footer_help(output),
            2 => self.end_time_edit_element.append_footer_help(output),
            3 => self.end_date_edit_element.append_footer_help(output),
            _ => {}
        }
    }

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>) {
        self.input_map.append_general_help(output);
        match self.selected_field {
            0 => self.start_time_edit_element.append_general_help(output),
            1 => self.start_date_edit_element.append_general_help(output),
            2 => self.end_time_edit_element.append_general_help(output),
            3 => self.end_date_edit_element.append_general_help(output),
            _ => {}
        }
    }
}
impl DialogWidget for TimeEntryWidget {
    type Output = Option<(DateTime<Utc>, DateTime<Utc>)>;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn handle_key(&mut self, key: KeyEvent) {
        let action = self.input_map.find_action(key);
        match action {
            None => self.handle_input_field_input(key),
            Some(a) => match a {
                TimeEntryActions::Next => self.select_next_field(),
                TimeEntryActions::Previous => self.select_previous_field(),
            },
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
            &self.start_date_edit_element,
            &self.start_time_edit_element,
            &self.start_date_error_msg,
        );

        // stop time heading
        self.draw_heading("End time", frame, inner_chunks[4]);
        self.draw_time_edit_fields(
            frame,
            inner_chunks[5],
            &self.end_date_edit_element,
            &self.end_time_edit_element,
            &self.end_date_error_msg,
        );
    }
}

#[cfg(test)]
mod time_entry_widget_tests {
    use super::*;
    use crate::widgets::test_helper::{char_key, key};

    #[test]
    fn time_entry_widget_tab_cycles_through_input_fields() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap();
        let mut widget = TimeEntryWidget::new(start, end);

        assert_eq!(widget.selected_field, 0);

        widget.handle_key(key(KeyCode::Tab));
        assert_eq!(widget.selected_field, 1);

        let tab_strokes = 3;
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
        assert_eq!(widget.selected_field, 3);
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

        widget.handle_key(key(KeyCode::Left));
        widget.handle_key(char_key('1'));
        assert!(widget.output().is_some());
    }

    #[test]
    fn time_entry_widget_start_time_before_end_time_returns_none() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 10, 1, 0).unwrap();

        let mut widget = TimeEntryWidget::new(start, end);

        for _ in 0..2 {
            widget.handle_key(key(KeyCode::Tab));
        }

        widget.handle_key(key(KeyCode::Down));
        widget.handle_key(key(KeyCode::Down));
        assert!(widget.output().is_none());

        widget.handle_key(key(KeyCode::Up));
        widget.handle_key(key(KeyCode::Up));
        assert!(widget.output().is_some());

        for _ in 0..2 {
            widget.handle_key(key(KeyCode::BackTab));
        }

        widget.handle_key(key(KeyCode::Up));
        widget.handle_key(key(KeyCode::Up));
        assert!(widget.output().is_none());
    }
}
