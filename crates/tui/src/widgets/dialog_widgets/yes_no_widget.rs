use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

enum YesNo {
    Yes,
    No,
}
pub struct YesNoWidget {
    decision: YesNo,
}

impl YesNoWidget {
    pub fn new() -> Self {
        Self {
            decision: YesNo::No,
        }
    }
}

impl DialogWidget for YesNoWidget {
    type Output = bool;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn get_help_text(&self) -> String {
        "<Enter>: Confirm | <Esc>: Cancel | <Left/Right> | [Y]es | [N]o".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                self.decision = match self.decision {
                    YesNo::Yes => YesNo::No,
                    YesNo::No => YesNo::Yes,
                }
            }
            KeyCode::Right => {
                self.decision = match self.decision {
                    YesNo::Yes => YesNo::No,
                    YesNo::No => YesNo::Yes,
                }
            }
            KeyCode::Char(c) => {
                if c == 'y' {
                    self.decision = YesNo::Yes;
                } else if c == 'n' {
                    self.decision = YesNo::No;
                }
            }
            _ => (),
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
