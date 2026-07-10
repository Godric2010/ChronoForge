use domain::types::DailyTimer;
use ratatui::layout::HorizontalAlignment::Center;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub struct ActiveTimer {
    daily_timer: DailyTimer,
}

impl ActiveTimer {
    pub fn new() -> Self {
        Self {
            daily_timer: Default::default(),
        }
    }

    pub fn set_passed_time(&mut self, passed_time: DailyTimer) {
        self.daily_timer = passed_time;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mut color = Color::Gray;

        let mut time_elapsed = self.daily_timer.elapsed_time_in_minutes;
        if let Some(active_timer_time) = self.daily_timer.currently_active_timer_elapsed_minutes {
            time_elapsed += active_timer_time;
            color = Color::Rgb(255, 125, 0);
        }

        let target_time_text = self.create_time_string(self.daily_timer.time_target_in_minutes);
        let elapsed_time_text = self.create_time_string(time_elapsed);

        let timer_text = format!("{} / {}", elapsed_time_text, target_time_text);
        let text_line = Line::from(timer_text).alignment(Center);

        let paragraph = Paragraph::new(text_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::LightDoubleDashed)
                .border_style(Style::default().fg(color)),
        );
        frame.render_widget(paragraph, area);
    }

    fn create_time_string(&self, time: u32) -> String {
        let hours = time / 60;
        let mins = time - hours * 60;
        format!("{:02}:{:02}", hours, mins)
    }
}
