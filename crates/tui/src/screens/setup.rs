use crate::app_action::SetupAction;
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::screens::dialog::{Dialog, DialogResult};
use crate::widgets::dialog_widgets::{PathWidget, ValidationMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use figlet_rs::FIGlet;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::path::PathBuf;

#[derive(PartialEq)]
struct ActionItem {
    name: String,
    action: Action,
}

impl ActionItem {
    pub fn render(&self, frame: &mut Frame, area: Rect, style: Style) {
        let name_paragraph = Paragraph::new(self.name.clone()).style(style);
        frame.render_widget(name_paragraph, area);
    }
}

#[derive(PartialEq)]
enum Action {
    Quit,
    CreateNewDatabase,
    LinkDatabase,
}

#[derive(Copy, Clone)]
enum SetupInputAction {
    Up,
    Down,
    Select,
}

pub struct SetupScreen {
    heading: String,
    heading_height: u16,
    action_items: Vec<ActionItem>,
    selected_action_idx: usize,
    dialog: Option<Dialog<PathWidget>>,
    default_style: Style,
    highlight_style: Style,
    input_map: InputMap<SetupInputAction>,
}

impl Default for SetupScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupScreen {
    pub fn new() -> Self {
        let font = FIGlet::standard().unwrap();
        let figure = font.convert("Chrono Forge").unwrap();
        let heading = figure.to_string();

        let input_map = InputMap::new(
            "Setup",
            vec![
                KeyBinding {
                    key_code: KeyCode::Up,
                    key_modifier: KeyModifiers::NONE,
                    key_name: "Up".to_string(),
                    key_description: "Go up".to_string(),
                    action: SetupInputAction::Up,
                    display_in_footer: false,
                },
                KeyBinding {
                    key_code: KeyCode::Down,
                    key_modifier: KeyModifiers::NONE,
                    key_name: "Down".to_string(),
                    key_description: "Go down".to_string(),
                    action: SetupInputAction::Down,
                    display_in_footer: false,
                },
                KeyBinding {
                    key_code: KeyCode::Enter,
                    key_modifier: KeyModifiers::NONE,
                    key_name: "Select".to_string(),
                    key_description: "Select action item".to_string(),
                    action: SetupInputAction::Select,
                    display_in_footer: false,
                },
            ],
        );

        Self {
            heading,
            heading_height: figure.height as u16,
            action_items: vec![
                ActionItem {
                    name: "Create new database".to_string(),
                    action: Action::CreateNewDatabase,
                },
                ActionItem {
                    name: "Link to existing database".to_string(),
                    action: Action::LinkDatabase,
                },
                ActionItem {
                    name: "Quit".to_string(),
                    action: Action::Quit,
                },
            ],
            selected_action_idx: 0,
            dialog: None,
            default_style: Style::default(),
            highlight_style: Style::default().add_modifier(Modifier::REVERSED),
            input_map,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(self.heading_height),
            Constraint::Percentage(10),
            Constraint::Length(self.action_items.len() as u16),
            Constraint::Min(1),
        ])
        .split(area);

        let heading = Paragraph::new(self.heading.clone()).alignment(Alignment::Center);
        frame.render_widget(heading, vertical[1]);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(vertical[3]);

        let action_items_rect = horizontal[1];
        for (idx, action_item) in self.action_items.iter().enumerate() {
            let rect = Rect::new(
                action_items_rect.x,
                action_items_rect.y + idx as u16,
                action_items_rect.width,
                1,
            );

            if idx == self.selected_action_idx {
                action_item.render(frame, rect, self.highlight_style);
            } else {
                action_item.render(frame, rect, self.default_style);
            };
        }

        if let Some(dialog) = &self.dialog {
            dialog.render(frame, area);
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<SetupAction> {
        if let Some(dialog) = &mut self.dialog {
            let result = dialog.handle_input(key_event);
            return self.handle_dialog_result(result);
        }

        let action = self.input_map.find_action(key_event);
        if let Some(input_action) = action {
            match input_action {
                SetupInputAction::Up => {
                    if self.selected_action_idx > 0 {
                        self.selected_action_idx -= 1;
                    }
                }
                SetupInputAction::Down => {
                    if self.selected_action_idx < self.action_items.len() - 1 {
                        self.selected_action_idx += 1;
                    }
                }
                SetupInputAction::Select => {
                    let action = &self.action_items[self.selected_action_idx].action;
                    match action {
                        Action::Quit => return Some(SetupAction::Quit),
                        Action::CreateNewDatabase => {
                            let widget = PathWidget::new(
                                None,
                                ValidationMode::WritableDirectory("chrono-forge.db".to_string()),
                            );
                            self.dialog = Some(Dialog::new("Create a new database at:", widget));
                        }
                        Action::LinkDatabase => {
                            let widget = PathWidget::new(
                                None,
                                ValidationMode::DirectoryContainsFile(
                                    "chrono-forge.db".to_string(),
                                ),
                            );
                            self.dialog = Some(Dialog::new("Link to existing database:", widget));
                        }
                    }
                }
            }
        }
        None
    }

    fn handle_dialog_result(&mut self, result: DialogResult<PathBuf>) -> Option<SetupAction> {
        match result {
            DialogResult::None => None,
            DialogResult::Cancelled => None,
            DialogResult::Confirmed(path_buf) => match self.selected_action_idx {
                0 => Some(SetupAction::CreateNewDatabase(path_buf)),
                1 => Some(SetupAction::LinkNewDatabase(path_buf)),
                _ => None,
            },
            DialogResult::Help(_) => None,
        }
    }
}
