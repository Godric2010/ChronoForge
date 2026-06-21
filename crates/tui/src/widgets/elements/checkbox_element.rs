use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::widgets::elements::{ElementSize, WidgetElement};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Clone, Copy)]
enum CheckboxAction {
    Toggle,
}
pub struct CheckboxElement {
    label: String,
    checked: bool,
    active: bool,
    size: ElementSize,
    input_map: InputMap<CheckboxAction>,
}

impl CheckboxElement {
    pub fn new(label: String, label_width: u16, checked: bool) -> Self {
        let key_bindings = vec![KeyBinding {
            key_code: KeyCode::Char(' '),
            key_modifier: KeyModifiers::empty(),
            key_name: "Space".to_string(),
            key_description: "Toggle".to_string(),
            action: CheckboxAction::Toggle,
            display_in_footer: true,
        }];
        let input_map = InputMap::new("Checkbox Actions", key_bindings);
        Self {
            size: ElementSize {
                width: label_width + 4,
                height: 1,
            },
            label,
            checked,
            active: false,
            input_map,
        }
    }
}

impl HelpProvider for CheckboxElement {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
    }
}

impl WidgetElement for CheckboxElement {
    type Output = bool;

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn get_size(&self) -> &ElementSize {
        &self.size
    }

    fn render(&self, frame: &mut Frame, pos_x: u16, pos_y: u16) {
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

    fn handle_key(&mut self, key: KeyEvent) {
        let action = self.input_map.find_action(key);
        if let Some(_action) = action {
            self.checked = !self.checked;
        }
    }

    fn get_output(&self) -> Self::Output {
        self.checked
    }
}
