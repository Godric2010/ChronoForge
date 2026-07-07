use crate::widgets::selectable_card_list::card_render_helper::*;
use crate::widgets::selectable_card_list::card_trait::SelectableCard;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

#[derive(Default)]
pub struct TaskCard {
    pub task_name: String,
    pub total_minutes: u32,
    pub time_limit: Option<u32>,
    pub is_archived: bool,
    is_selected: bool,
    normal_style: Style,
    archived_style: Style,
}

impl TaskCard {
    pub fn new(
        task_name: String,
        total_minutes: u32,
        time_limit: Option<u32>,
        is_archived: bool,
    ) -> Self {
        let normal_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::White);
        let archived_style = Style::default()
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
            .fg(Color::Gray);

        Self {
            task_name,
            total_minutes,
            time_limit,
            is_archived,
            is_selected: false,
            normal_style,
            archived_style,
        }
    }
    fn render_task_info(&self, area: Rect, frame: &mut Frame) {
        let info_chunks = Layout::vertical([
            Constraint::Min(1),    // spacer
            Constraint::Length(1), // task name
            Constraint::Min(1),    // spacer
        ])
        .split(area);

        let name: String;
        let style: Style;
        if self.is_archived {
            name = format!("[Done] {}", self.task_name);
            style = self.archived_style;
        } else {
            name = self.task_name.clone();
            style = self.normal_style;
        }
        let name_paragraph = Paragraph::new(name).style(style);
        frame.render_widget(name_paragraph, info_chunks[1]);
    }
}

impl SelectableCard for TaskCard {
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

        self.render_task_info(info_side, frame);
        if let Some(time_limit) = self.time_limit {
            render_time_info_with_time_limit(time_side, frame, self.total_minutes, time_limit);
        } else {
            render_time_info(time_side, frame, self.total_minutes);
        }
    }
}
