use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::ui_error_message::UiErrorMessage;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Clone, Copy)]
enum ErrorWidgetActions {
    Confirm,
}
pub struct ErrorWidget {
    error_message: UiErrorMessage,
    should_close: bool,
    input_map: InputMap<ErrorWidgetActions>,
}

impl ErrorWidget {
    pub fn new(error_message: UiErrorMessage) -> Self {
        let key_bindings = vec![KeyBinding {
            key_code: KeyCode::Enter,
            key_modifier: KeyModifiers::empty(),
            key_name: "↲".to_string(),
            key_description: "Confirm the error and close the dialog".to_string(),
            action: ErrorWidgetActions::Confirm,
            display_in_footer: true,
        }];
        let input_map = InputMap::new("Error Dialog Actions", key_bindings);
        Self {
            error_message,
            should_close: false,
            input_map,
        }
    }
}
impl HelpProvider for ErrorWidget {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
    }
}
impl DialogWidget for ErrorWidget {
    type Output = ();

    fn get_type(&self) -> WidgetType {
        WidgetType::Error
    }

    fn handle_key(&mut self, key: KeyEvent) {
        let action = self.input_map.find_action(key);
        match action {
            Some(ErrorWidgetActions::Confirm) => self.should_close = true,
            None => {}
        }
    }

    fn output(&self) -> Self::Output {}

    fn height(&self) -> u16 {
        4
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

        let title_paragraph = Paragraph::new(self.error_message.get_title())
            .style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(title_paragraph, vertical[0]);

        let message_paragraph = Paragraph::new(self.error_message.get_message());
        frame.render_widget(message_paragraph, vertical[1]);

        let confirm_paragraph = Paragraph::new("OK")
            .centered()
            .style(Style::default().add_modifier(Modifier::REVERSED));

        let horizontal = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(2),
            Constraint::Min(1),
        ])
        .split(vertical[3]);
        frame.render_widget(confirm_paragraph, horizontal[1]);
    }
}
