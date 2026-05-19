use crate::widgets::selectable_card_list::card_trait::SelectableCard;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

#[derive(Default)]
pub struct ProjectCard {
    pub project_name: String,
    pub total_tasks: usize,
    pub total_minutes: u32,
    is_selected: bool,
}

impl ProjectCard {
    pub fn new(project_name: String, total_tasks: usize, total_minutes: u32) -> Self {
        Self {
            project_name,
            total_tasks,
            total_minutes,
            is_selected: false,
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

        let name_paragraph = Paragraph::new(self.project_name.clone()).style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        );
        frame.render_widget(name_paragraph, info_chunks[1]);

        let task_paragraph = Paragraph::new(format!("Tasks: {}", self.total_tasks));
        frame.render_widget(task_paragraph, info_chunks[3]);
    }

    fn render_time_info(&self, area: Rect, frame: &mut Frame) {

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
            self.total_minutes / 60,
            self.total_minutes % 60
        ))
        .alignment(HorizontalAlignment::Center);
        frame.render_widget(time_paragraph, time_chunks[1]);
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
        self.render_time_info(time_side, frame);
    }
}
