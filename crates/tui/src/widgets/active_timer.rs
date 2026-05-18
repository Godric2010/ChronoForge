use ratatui::layout::HorizontalAlignment::Center;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub struct ActiveTimer {
    pub passed_time: Option<u32>,
}

impl ActiveTimer {
    pub fn new() -> Self {
        Self { passed_time: None }
    }

    pub fn set_passed_time(&mut self, passed_time: Option<u32>) {
        self.passed_time = passed_time;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let time_text = self.create_time_string();
        let text_line = Line::from(time_text.as_str()).alignment(Center);

        let mut color = Color::Gray;
        if self.passed_time.is_some() {
            color = Color::Rgb(255, 125, 0);
        }

        let paragraph = Paragraph::new(text_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::LightDoubleDashed)
                .border_style(Style::default().fg(color)),
        );
        frame.render_widget(paragraph, area);
    }

    fn create_time_string(&self) -> String {
        if let Some(time) = self.passed_time {
            let hours = time / 60;
            let mins = time - hours * 60;
            return format!("{:02}:{:02}", hours, mins);
        }
        "00:00".to_string()
    }
}
