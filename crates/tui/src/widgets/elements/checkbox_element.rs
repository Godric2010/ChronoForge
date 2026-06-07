use crate::widgets::elements::ElementSize;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct CheckboxElement {
    label: String,
    checked: bool,
    active: bool,
    size: ElementSize,
}

impl CheckboxElement {
    pub fn new(label: String, label_width: u16, checked: bool) -> Self {
        Self {
            size: ElementSize {
                width: label_width + 4,
                height: 1,
            },
            label,
            checked,
            active: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn get_size(&self) -> &ElementSize {
        &self.size
    }

    pub fn render(&mut self, frame: &mut Frame, pos_x: u16, pos_y: u16) {
        let rect = Rect::new(pos_x, pos_y, self.size.width, self.size.height);
        let horizontal = Layout::horizontal([
            Constraint::Length(self.size.width - 4),
            Constraint::Length(4),
        ])
        .split(rect);

        let label_paragraph = Paragraph::new(self.label.clone());
        frame.render_widget(label_paragraph, horizontal[0]);

        let checkbox_string = if self.checked { "[X]" } else { "[ ]" };
        let checkbox_paragraph = Paragraph::new(checkbox_string).right_aligned();
        frame.render_widget(checkbox_paragraph, horizontal[1]);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char(' ') {
            self.checked = !self.checked;
        }
    }
}
