use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_time_info(area: Rect, frame: &mut Frame, total_minutes: u32) {
    let inner = render_time_info_block(area, frame);

    let time_chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(inner);

    let time_paragraph = Paragraph::new(format!(
        "{:02}:{:02}",
        total_minutes / 60,
        total_minutes % 60
    ))
    .alignment(HorizontalAlignment::Center);
    frame.render_widget(time_paragraph, time_chunks[1]);
}
pub fn render_time_info_with_time_limit(
    area: Rect,
    frame: &mut Frame,
    total_minutes: u32,
    time_limit: u32,
) {
    let inner = render_time_info_block(area, frame);
    let time_chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(inner);

    let color = if time_limit > total_minutes {
        Color::White
    } else {
        Color::Red
    };

    let time_paragraph = Paragraph::new(format!(
        "{:02}:{:02}",
        total_minutes / 60,
        total_minutes % 60
    ))
    .alignment(HorizontalAlignment::Center)
    .style(Style::default().fg(color));
    frame.render_widget(time_paragraph, time_chunks[1]);

    let time_limit_paragraph =
        Paragraph::new(format!("({:02}:{:02})", time_limit / 60, time_limit % 60))
            .alignment(HorizontalAlignment::Center);
    frame.render_widget(time_limit_paragraph, time_chunks[3]);
}

fn render_time_info_block(area: Rect, frame: &mut Frame) -> Rect {
    let time_info_block = Block::default()
        .borders(Borders::LEFT)
        .border_type(BorderType::LightTripleDashed);

    let inner = time_info_block.inner(area);
    frame.render_widget(time_info_block, area);
    inner
}
