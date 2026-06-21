use crate::input::help_context::KeyBindingHelpContext;
use crate::input::HelpProvider;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crate::widgets::elements::{InputMode, TextEditElement, WidgetElement};
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct PathWidget {
    path_input: TextEditElement,
}

impl PathWidget {
    pub fn new() -> Self {
        Self {
            path_input: TextEditElement::new(None, InputMode::Ascii),
        }
    }
}

impl HelpProvider for PathWidget {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.path_input.append_footer_help(output);
    }
}

impl DialogWidget for PathWidget {
    type Output = String;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn handle_key(&mut self, key: KeyEvent) {
        self.path_input.handle_key(key);
    }

    fn output(&self) -> Self::Output {
        self.path_input.get_output().to_string()
    }

    fn height(&self) -> u16 {
        self.path_input.get_size().height
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.path_input.render(frame, area.x, area.y);
    }
}
