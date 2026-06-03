use crate::widgets::selectable_card_list::card_trait::SelectableCard;
use chrono::{DateTime, Datelike, Local, Utc};
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

#[derive(Default)]
pub struct TimeEntryCard {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    duration_min: u32,
    duration_days: u16,
    weekday: String,
    older_than_week: bool,
    is_selected: bool,
}

impl TimeEntryCard {
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        let duration_min = (end_time - start_time).num_minutes() as u32;
        let duration_days = (end_time - start_time).num_days() as u16;
        let weekday = start_time.weekday().to_string();
        let older_than_week = (Utc::now() - start_time).num_days() >= 7;

        Self {
            start_time: start_time.with_timezone(&Local),
            end_time: end_time.with_timezone(&Local),
            duration_min,
            duration_days,
            weekday,
            older_than_week,
            is_selected: false,
        }
    }
    fn render_time_entry_info(&self, area: Rect, frame: &mut Frame) {
        let info_chunks = Layout::vertical([
            Constraint::Length(1), // spacer
            Constraint::Length(1), // weekday
            Constraint::Length(1), // time start - time end
            Constraint::Length(1), // spacer
        ])
        .split(area);

        let weekday_text = if self.older_than_week {
            let start_time_day = self.start_time.day();
            let start_time_month = self.start_time.month();
            let start_time_year = self.start_time.year();
            format!(
                "{} ({:02}/{:02}/{:04})",
                self.weekday, start_time_day, start_time_month, start_time_year
            )
        } else {
            self.weekday.to_string()
        };

        let weekday_paragraph = Paragraph::new(weekday_text).style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        );
        frame.render_widget(weekday_paragraph, info_chunks[1]);

        let start_time_string = self.start_time.format("%H:%M").to_string();
        let end_time_string = self.end_time.format("%H:%M").to_string();
        let day_appendage = if self.duration_days > 0 {
            format!("(+{})", self.duration_days)
        } else {
            String::from("")
        };

        let time_entry_paragraph = Paragraph::new(format!(
            "{} - {} {}",
            start_time_string, end_time_string, day_appendage
        ));
        frame.render_widget(time_entry_paragraph, info_chunks[2]);
    }

    fn render_duration_info(&self, area: Rect, frame: &mut Frame) {
        let time_info_block = Block::default()
            .borders(Borders::LEFT)
            .border_type(BorderType::LightTripleDashed);

        let inner = time_info_block.inner(area);
        frame.render_widget(time_info_block, area);

        let time_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(inner);

        let time_paragraph = Paragraph::new(format!(
            "{:02}:{:02}",
            self.duration_min / 60,
            self.duration_min % 60
        ))
        .alignment(HorizontalAlignment::Center);
        frame.render_widget(time_paragraph, time_chunks[1]);
    }
}

impl SelectableCard for TimeEntryCard {
    fn enable_highlight(&mut self) {
        self.is_selected = true;
    }

    fn disable_highlight(&mut self) {
        self.is_selected = false;
    }

    fn render(&self, frame: &mut Frame, rect: Rect) {
        let border_color = if self.is_selected {
            Color::Rgb(255, 125, 0)
        } else {
            Color::Gray
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::LightTripleDashed)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(rect);
        frame.render_widget(block, rect);

        let vertical_chunks = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Percentage(80),
            Constraint::Min(1),
        ])
        .split(inner);
        let info_side = vertical_chunks[1];
        let time_side = vertical_chunks[2];

        self.render_time_entry_info(info_side, frame);
        self.render_duration_info(time_side, frame);
    }
}
