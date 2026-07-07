use crate::widgets::selectable_card_list::card_render_helper::*;
use crate::widgets::selectable_card_list::card_trait::SelectableCard;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

#[derive(Default)]
pub struct ProjectCard {
    pub project_name: String,
    pub total_tasks: usize,
    pub total_minutes: u32,
    pub time_limit: Option<u32>,
    pub is_archived: bool,
    is_selected: bool,
    normal_style: Style,
    archived_style: Style,
}

impl ProjectCard {
    pub fn new(
        project_name: String,
        total_tasks: usize,
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
            project_name,
            total_tasks,
            total_minutes,
            time_limit,
            is_archived,
            is_selected: false,
            normal_style,
            archived_style,
        }
    }
    fn render_project_info(&self, area: Rect, frame: &mut Frame) {
        let info_chunks = Layout::vertical([
            Constraint::Length(1), // spacer
            Constraint::Length(1), // project name
            Constraint::Length(1), // tasks in project
            Constraint::Length(1), // spacer
        ])
        .split(area);

        let style: Style;
        let name_text: String;
        if self.is_archived {
            style = self.archived_style;
            name_text = format!("{} (archived)", self.project_name);
        } else {
            style = self.normal_style;
            name_text = self.project_name.clone();
        }

        let name_paragraph = Paragraph::new(name_text).style(style);
        frame.render_widget(name_paragraph, info_chunks[1]);

        let task_paragraph = Paragraph::new(format!("Tasks: {}", self.total_tasks));
        frame.render_widget(task_paragraph, info_chunks[3]);
    }
}

impl SelectableCard for ProjectCard {
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

        self.render_project_info(info_side, frame);
        if let Some(time_limit) = self.time_limit {
            render_time_info_with_time_limit(time_side, frame, self.total_minutes, time_limit);
        } else {
            render_time_info(time_side, frame, self.total_minutes);
        }
    }
}
