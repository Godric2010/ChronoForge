use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use uuid::Uuid;

#[derive(Clone, Copy)]
enum ListWidgetActions {
    Next,
    Previous,
}

pub struct ListItem {
    pub(crate) name: String,
    pub(crate) id: Uuid,
}
pub struct ListWidget {
    items: Vec<ListItem>,
    selected_index: usize,
    scroll_offset: usize,
    visible_count: usize,
    input_map: InputMap<ListWidgetActions>,
}

impl ListWidget {
    pub fn new(items: Vec<ListItem>) -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Down,
                key_modifier: KeyModifiers::empty(),
                key_name: "↓".to_string(),
                key_description: "Select the next item in the list".to_string(),
                action: ListWidgetActions::Next,
                display_in_footer: false,
            },
            KeyBinding {
                key_code: KeyCode::Up,
                key_modifier: KeyModifiers::empty(),
                key_name: "↑".to_string(),
                key_description: "Select the previous item in the list".to_string(),
                action: ListWidgetActions::Previous,
                display_in_footer: false,
            },
        ];
        let input_map = InputMap::new("List Actions", key_bindings);
        Self {
            items,
            selected_index: 0,
            scroll_offset: 0,
            visible_count: 8,
            input_map,
        }
    }

    fn render_top_more_text(&self, frame: &mut Frame, area: Rect) {
        let more_text_top: String = if self.scroll_offset > 0 {
            "↑ more".to_string()
        } else {
            String::new()
        };
        let top_paragraph =
            Paragraph::new(more_text_top).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(top_paragraph, area);
    }
    fn render_list_items(&mut self, frame: &mut Frame, area: Rect) {
        for index in 0..self.visible_count {
            let item_index = index + self.scroll_offset;
            if item_index >= self.items.len() {
                break;
            }

            let item_area = Rect {
                x: area.x,
                y: area.y + index as u16,
                width: area.width,
                height: 1,
            };

            let paragraph = if self.selected_index == item_index {
                Paragraph::new(self.items[item_index].name.clone())
                    .style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                Paragraph::new(self.items[item_index].name.clone())
            };

            frame.render_widget(paragraph, item_area);
        }
    }
    fn render_bottom_more_text(&self, frame: &mut Frame, area: Rect) {
        let more_text_bottom: String = if self.scroll_offset + self.visible_count < self.items.len()
        {
            "↓ more".to_string()
        } else {
            String::new()
        };
        let bottom_paragraph =
            Paragraph::new(more_text_bottom).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(bottom_paragraph, area);
    }

    fn select_next_item(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected_index = (self.selected_index + 1).min(self.items.len() - 1);
        if self.selected_index >= self.visible_count + self.scroll_offset {
            self.scroll_offset += 1;
        }
    }
    fn select_previous_item(&mut self) {
        if self.selected_index == 0 {
            return;
        }
        self.selected_index -= 1;
        if self.selected_index < self.scroll_offset {
            self.scroll_offset -= 1;
        }
    }
}

impl DialogWidget for ListWidget {
    type Output = Option<Uuid>;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn render_input_map_help(&self) -> String {
        "<Enter>: Confirm | <Esc>: Cancel | <Up/Down>".to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        let action = self.input_map.find_action(key);
        match action {
            None => {}
            Some(a) => match a {
                ListWidgetActions::Next => self.select_next_item(),
                ListWidgetActions::Previous => self.select_previous_item(),
            },
        }
    }

    fn output(&self) -> Self::Output {
        self.items.get(self.selected_index).map(|item| item.id)
    }

    fn height(&self) -> u16 {
        10
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(self.visible_count as u16),
            Constraint::Length(1),
        ])
        .split(area);

        self.render_top_more_text(frame, vertical[0]);
        self.render_list_items(frame, vertical[1]);
        self.render_bottom_more_text(frame, vertical[2]);
    }
}

#[cfg(test)]
mod list_widget_tests {
    use super::*;
    use crate::widgets::test_helper::key;

    #[test]
    fn list_widget_down_selects_next_up_selects_prev_item() {
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();

        let mut widget = ListWidget::new(vec![
            ListItem {
                name: "First".to_string(),
                id: first,
            },
            ListItem {
                name: "Second".to_string(),
                id: second,
            },
        ]);

        assert_eq!(widget.output().unwrap(), first);

        widget.handle_key(key(KeyCode::Down));
        assert_eq!(widget.output().unwrap(), second);

        widget.handle_key(key(KeyCode::Up));
        assert_eq!(widget.output().unwrap(), first);
    }

    #[test]
    fn list_widget_up_at_top_does_not_move() {
        let first = Uuid::new_v4();
        let mut widget = ListWidget::new(vec![ListItem {
            name: "First".to_string(),
            id: first,
        }]);

        widget.handle_key(key(KeyCode::Up));
        assert_eq!(widget.output().unwrap(), first);
    }

    #[test]
    fn list_widget_down_at_bottom_does_not_move() {
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        let mut widget = ListWidget::new(vec![
            ListItem {
                name: "First".to_string(),
                id: first,
            },
            ListItem {
                name: "Second".to_string(),
                id: second,
            },
        ]);

        widget.handle_key(key(KeyCode::Down));
        widget.handle_key(key(KeyCode::Down));
        widget.handle_key(key(KeyCode::Down));
        assert_eq!(widget.output().unwrap(), second);
    }

    #[test]
    fn list_widget_empty_list_down_does_not_crash() {
        let mut widget = ListWidget::new(vec![]);
        widget.handle_key(key(KeyCode::Down));
        assert_eq!(widget.output(), None);
    }

    #[test]
    fn list_widget_empty_list_up_does_not_crash() {
        let mut widget = ListWidget::new(vec![]);
        widget.handle_key(key(KeyCode::Up));
        assert_eq!(widget.output(), None);
    }
}
