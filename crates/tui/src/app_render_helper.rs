use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::Paragraph;
use ratatui::{symbols, Frame};

pub fn render_terminal_too_small_text(frame: &mut Frame, area: Rect) {
    let vertical_layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(area);
    let text = Paragraph::new("Window too small. At least 150x40 required").centered();

    frame.render_widget(text, vertical_layout[1]);
}
pub fn render_separator(frame: &mut Frame, area: Rect) {
    let separator = symbols::line::HORIZONTAL.repeat(area.width.saturating_sub(2) as usize);
    let separator_widget = Paragraph::new(Line::from(separator));
    let mut rect = area;
    rect.x += 1;
    frame.render_widget(separator_widget, rect);
}
