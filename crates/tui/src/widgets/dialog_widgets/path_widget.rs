use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
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
        let mut text_edit_element = TextEditElement::new(None, InputMode::Ascii);
        text_edit_element.set_active(true);
        Self {
            path_input: text_edit_element,
        }
    }
}

impl HelpProvider for PathWidget {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.path_input.append_footer_help(output);
    }

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>) {
        self.path_input.append_general_help(output);
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

    fn render(&self, frame: &mut Frame, area: Rect) {
        self.path_input.render(frame, area.x, area.y);
    }
}
