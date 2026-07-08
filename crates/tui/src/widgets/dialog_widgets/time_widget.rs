use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::HelpProvider;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crate::widgets::elements::{TimeEditElement, WidgetElement};
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

pub struct TimeWidget {
    time_edit_element: TimeEditElement,
}

impl TimeWidget {
    pub fn new(time_value_minutes: u32) -> Self {
        let hours = time_value_minutes / 60;
        let minutes = time_value_minutes % 60;

        let mut time_edit_element = TimeEditElement::new(hours, minutes, Some(24));
        time_edit_element.set_active(true);

        Self { time_edit_element }
    }
}

impl HelpProvider for TimeWidget {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.time_edit_element.append_footer_help(output);
    }

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>) {
        self.time_edit_element.append_general_help(output);
    }
}

impl DialogWidget for TimeWidget {
    type Output = u32;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn handle_key(&mut self, key: KeyEvent) {
        self.time_edit_element.handle_key(key);
    }

    fn output(&self) -> Self::Output {
        let time = self.time_edit_element.get_output();
        time.hour * 60 + time.minute
    }

    fn height(&self) -> u16 {
        1
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

        let time_edit_center = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(inner_chunks[1])[1];

        self.time_edit_element
            .render(frame, time_edit_center.x, time_edit_center.y);
    }
}
