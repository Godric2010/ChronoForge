use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crate::widgets::elements::{CheckboxElement, InputMode, TextEditElement, TimeEditElement};
use crossterm::event::{KeyCode, KeyEvent};
use domain::types::Task;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::Block;
use ratatui::Frame;

pub struct TaskEditOutput {
    pub task_name: String,
    pub time_limit: Option<u32>,
}

pub struct TaskEditWidget {
    name_input: TextEditElement,
    time_limit_checkbox: CheckboxElement,
    time_limit_input: TimeEditElement,

    active_element_index: u8,
}

impl TaskEditWidget {
    pub fn empty() -> Self {
        let mut name_input = TextEditElement::new(None, InputMode::Naming);
        name_input.set_active(true);
        Self {
            name_input,
            time_limit_checkbox: CheckboxElement::new("Set time limit:".to_string(), 20, false),
            time_limit_input: TimeEditElement::new(0, 0, None),
            active_element_index: 0,
        }
    }

    pub fn new(task: &Task) -> Self {
        let has_time_limit: bool;
        let time_limit_h: u32;
        let time_limit_m: u32;
        match task.time_limit {
            None => {
                has_time_limit = false;
                time_limit_h = 0;
                time_limit_m = 0;
            }
            Some(limit_in_minutes) => {
                has_time_limit = true;
                time_limit_h = limit_in_minutes / 60;
                time_limit_m = limit_in_minutes % 60;
            }
        }

        let mut name_input = TextEditElement::new(Some(task.name.clone()), InputMode::Naming);
        name_input.set_active(true);

        Self {
            name_input,
            time_limit_checkbox: CheckboxElement::new(
                "Set time limit:".to_string(),
                20,
                has_time_limit,
            ),
            time_limit_input: TimeEditElement::new(time_limit_h, time_limit_m, None),
            active_element_index: 0,
        }
    }

    fn enable_field(&mut self) {
        let idx = self.active_element_index;
        match idx {
            0 => {
                self.name_input.set_active(true);
                self.time_limit_checkbox.set_active(false);
                self.time_limit_input.set_active(false);
            }
            1 => {
                self.name_input.set_active(false);
                self.time_limit_checkbox.set_active(true);
                self.time_limit_input.set_active(false);
            }
            2 => {
                self.name_input.set_active(false);
                self.time_limit_checkbox.set_active(false);
                self.time_limit_input.set_active(true);
            }
            _ => {}
        }
    }
}

impl DialogWidget for TaskEditWidget {
    type Output = TaskEditOutput;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn get_help_text(&self) -> String {
        "<Esc>: Cancel | <Enter>: Confirm | <Tab>: Next".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        let max_element_idx: u8 = if self.time_limit_checkbox.is_checked() {
            3
        } else {
            2
        };

        match key.code {
            KeyCode::Tab => {
                self.active_element_index += 1;
                if self.active_element_index >= max_element_idx {
                    self.active_element_index = 0;
                }
                self.enable_field();
            }
            KeyCode::BackTab => {
                if self.active_element_index > 0 {
                    self.active_element_index -= 1;
                } else {
                    self.active_element_index = max_element_idx;
                }
                self.enable_field();
            }
            _ => match self.active_element_index {
                0 => self.name_input.handle_key(key),
                1 => self.time_limit_checkbox.handle_key(key),
                2 => self.time_limit_input.handle_key(key),
                _ => {}
            },
        }
    }

    fn output(&self) -> Self::Output {
        let time_limit = if self.time_limit_checkbox.is_checked() {
            Some(
                self.time_limit_input.get_hour_value() * 60
                    + self.time_limit_input.get_minute_value(),
            )
        } else {
            None
        };

        TaskEditOutput {
            task_name: self.name_input.get_content().to_string(),
            time_limit,
        }
    }

    fn height(&self) -> u16 {
        let border_height = 2;

        let name_input_height = self.name_input.get_size().height + border_height;
        let checkbox_height = self.time_limit_checkbox.get_size().height + border_height;
        let time_input_height = self.time_limit_input.get_size().height + border_height;
        name_input_height + checkbox_height + time_input_height
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(self.name_input.get_size().height + 2),
            Constraint::Length(self.time_limit_checkbox.get_size().height + 2),
            Constraint::Length(self.time_limit_input.get_size().height + 2),
        ])
        .split(area);

        let name_block = Block::default()
            .title("Task Name")
            .title_style(Style::default().add_modifier(Modifier::UNDERLINED))
            .style(Style::default().fg(if self.active_element_index == 0 {
                Color::Rgb(255, 125, 0)
            } else {
                Color::White
            }));
        let inner_name = name_block.inner(vertical[0]);
        frame.render_widget(name_block, vertical[0]);
        self.name_input.render(frame, inner_name.x, inner_name.y);

        let checkbox_block =
            Block::default().style(Style::default().fg(if self.active_element_index == 1 {
                Color::Rgb(255, 125, 0)
            } else {
                Color::White
            }));
        let inner_checkbox = checkbox_block.inner(vertical[1]);
        frame.render_widget(checkbox_block, vertical[1]);
        self.time_limit_checkbox
            .render(frame, inner_checkbox.x, inner_checkbox.y);

        if self.time_limit_checkbox.is_checked() {
            let time_edit_block = Block::default()
                .title("Time Limit")
                .title_style(Style::default().add_modifier(Modifier::UNDERLINED))
                .style(Style::default().fg(if self.active_element_index == 2 {
                    Color::Rgb(255, 125, 0)
                } else {
                    Color::White
                }));
            let inner_time = time_edit_block.inner(vertical[2]);
            frame.render_widget(time_edit_block, vertical[2]);
            self.time_limit_input
                .render(frame, inner_time.x, inner_time.y);
        }
    }
}
