use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Copy, Clone)]
enum YesNoActions {
    SetYes,
    SetNo,
    Toggle,
}

#[derive(Copy, Clone)]
enum YesNo {
    Yes,
    No,
}
pub struct YesNoWidget {
    decision: YesNo,
    input_map: InputMap<YesNoActions>,
}

impl YesNoWidget {
    pub fn new() -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Left,
                key_modifier: KeyModifiers::empty(),
                key_name: "←".to_string(),
                key_description: "Toggle decision".to_string(),
                action: YesNoActions::Toggle,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Right,
                key_modifier: KeyModifiers::empty(),
                key_name: "→".to_string(),
                key_description: "Toggle decision".to_string(),
                action: YesNoActions::Toggle,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Char('y'),
                key_modifier: KeyModifiers::empty(),
                key_name: "y".to_string(),
                key_description: "Yes".to_string(),
                action: YesNoActions::SetYes,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Char('n'),
                key_modifier: KeyModifiers::empty(),
                key_name: "n".to_string(),
                key_description: "No".to_string(),
                action: YesNoActions::SetNo,
                display_in_footer: true,
            },
        ];

        let input_map = InputMap::new("Yes No Actions", key_bindings);

        Self {
            decision: YesNo::No,
            input_map,
        }
    }

    fn toggle_yes_no(&mut self) {
        self.decision = match self.decision {
            YesNo::Yes => YesNo::No,
            YesNo::No => YesNo::Yes,
        }
    }

    fn set_no(&mut self) {
        self.decision = YesNo::No;
    }

    fn set_yes(&mut self) {
        self.decision = YesNo::Yes;
    }
}

impl HelpProvider for YesNoWidget {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        self.input_map.append_footer_help(output);
    }

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>) {
        self.input_map.append_general_help(output);
    }
}

impl DialogWidget for YesNoWidget {
    type Output = bool;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn handle_key(&mut self, key: KeyEvent) {
        let action = self.input_map.find_action(key);
        match action {
            None => {}
            Some(a) => match a {
                YesNoActions::SetYes => self.set_yes(),
                YesNoActions::SetNo => self.set_no(),
                YesNoActions::Toggle => self.toggle_yes_no(),
            },
        }
    }

    fn output(&self) -> Self::Output {
        match self.decision {
            YesNo::Yes => true,
            YesNo::No => false,
        }
    }

    fn height(&self) -> u16 {
        1
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let horizontal = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

        let styles = match self.decision {
            YesNo::Yes => {
                vec![
                    Style::default().add_modifier(Modifier::REVERSED),
                    Style::default(),
                ]
            }
            YesNo::No => {
                vec![
                    Style::default(),
                    Style::default().add_modifier(Modifier::REVERSED),
                ]
            }
        };

        let yes_paragraph = Paragraph::new("Yes").style(styles[0]);
        let no_paragraph = Paragraph::new("No").style(styles[1]);

        frame.render_widget(yes_paragraph, horizontal[1]);
        frame.render_widget(no_paragraph, horizontal[3]);
    }
}

#[cfg(test)]
mod yes_no_widget_tests {
    use super::*;
    use crate::widgets::test_helper::{char_key, key};

    #[test]
    fn yes_no_defaults_to_no() {
        let widget = YesNoWidget::new();
        assert!(!widget.output())
    }

    #[test]
    fn yes_no_widget_accepts_y_and_n() {
        let mut widget = YesNoWidget::new();
        widget.handle_key(char_key('y'));
        assert!(widget.output());

        widget.handle_key(char_key('n'));
        assert!(!widget.output());
    }

    #[test]
    fn yes_no_widget_toggles_with_left_and_right() {
        let mut widget = YesNoWidget::new();
        assert!(!widget.output());

        widget.handle_key(key(KeyCode::Left));
        assert!(widget.output());

        widget.handle_key(key(KeyCode::Right));
        assert!(!widget.output());
    }

    #[test]
    fn yes_no_widget_ignores_unknown_keys() {
        let mut widget = YesNoWidget::new();

        widget.handle_key(key(KeyCode::Tab));
        assert!(!widget.output());

        widget.handle_key(key(KeyCode::Enter));
        assert!(!widget.output());

        widget.handle_key(key(KeyCode::Esc));
        assert!(!widget.output());

        widget.handle_key(char_key('x'));
        assert!(!widget.output());
    }
}
