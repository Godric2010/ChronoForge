use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::{symbols, Frame};

pub enum DialogResult<T> {
    None,
    Cancelled,
    Confirmed(T),
}
pub struct Dialog<Widget: DialogWidget> {
    title: String,
    widget: Widget,
    height: u16,
}

impl<Widget: DialogWidget> Dialog<Widget> {
    pub fn new(title: &str, widget: Widget) -> Self {
        Self {
            title: String::from(title),
            height: 6 + &widget.height(),
            widget,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        // outer block
        let frame_color = match self.widget.get_type() {
            WidgetType::Input => Color::Gray,
            WidgetType::Error => Color::Red,
        };
        let outer_block = Block::default()
            .title(format!("< {} >", self.title))
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(frame_color));
        frame.render_widget(outer_block, dialog_draw_rect);

        // inner blocks
        let inner_chunks = Layout::vertical([
            Constraint::Length(1),                    // border
            Constraint::Length(1),                    // spacer
            Constraint::Length(self.widget.height()), // widget
            Constraint::Length(1),                    // spacer
            Constraint::Length(1),                    // separator
            Constraint::Length(1),                    // help text
        ])
        .split(dialog_draw_rect);

        let mut widget_rect = inner_chunks[2];
        widget_rect.width -= 2;
        widget_rect.x += 1;

        self.widget.render(frame, widget_rect);
        self.render_separator(frame, inner_chunks[4]);
        self.render_help_text(frame, inner_chunks[5]);
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> DialogResult<Widget::Output> {
        match key_event.code {
            KeyCode::Enter => DialogResult::Confirmed(self.widget.output()),
            KeyCode::Esc => DialogResult::Cancelled,
            _ => {
                self.widget.handle_key(key_event);
                DialogResult::None
            }
        }
    }

    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let vertical_chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(self.height),
            Constraint::Min(0),
        ])
        .split(area);

        let dialog_row = vertical_chunks[1];

        let horizontal_chunks = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Percentage(30),
            Constraint::Min(0),
        ])
        .split(dialog_row);

        horizontal_chunks[1]
    }

    fn render_separator(&self, frame: &mut Frame, area: Rect) {
        let separator = symbols::line::HORIZONTAL.repeat(area.width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = area;
        rect.x += 1;
        frame.render_widget(separator_widget, rect);
    }

    fn render_help_text(&self, frame: &mut Frame, area: Rect) {
        let help_box = Paragraph::new(self.widget.get_help_text()).alignment(Alignment::Center);
        frame.render_widget(help_box, area);
    }
}
