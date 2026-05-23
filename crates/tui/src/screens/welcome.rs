use figlet_rs::FIGlet;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct WelcomeScreen {
    text: String,
    height: u16,
}

impl WelcomeScreen {
    pub fn new(text: String) -> Self {
        let font = FIGlet::standard().unwrap();
        let figure = font.convert(&text).unwrap();
        let display_text = figure.to_string();

        Self {
            text: display_text,
            height: figure.height as u16,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(self.height),
            Constraint::Min(1),
        ])
        .split(area);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(vertical[1]);

        let mut content_area = horizontal[1];
        content_area.y -= self.height / 2;

        let paragraph = Paragraph::new(self.text.clone()).alignment(Alignment::Center);
        frame.render_widget(paragraph, content_area);
    }
}
