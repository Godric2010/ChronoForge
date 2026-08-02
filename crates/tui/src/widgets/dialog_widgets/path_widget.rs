use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::HelpProvider;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crate::widgets::elements::{InputMode, TextEditElement, WidgetElement};
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::path::PathBuf;

#[derive(Clone)]
pub enum ValidationMode {
    DirectoryContainsFile(String),
    WritableDirectory(String),
}

#[derive(Clone)]
enum ValidationState {
    Empty,
    Ok,
    Warning(String),
    Invalid(String),
}
pub struct PathWidget {
    path_input: TextEditElement,
    mode: ValidationMode,
    state: ValidationState,
}

impl PathWidget {
    pub fn new(initial_text: Option<String>, mode: ValidationMode) -> Self {
        let initial_state = if initial_text.is_none() {
            ValidationState::Empty
        } else {
            ValidationState::Ok
        };
        let mut text_edit_element = TextEditElement::new(initial_text, InputMode::Ascii);
        text_edit_element.set_active(true);
        Self {
            path_input: text_edit_element,
            mode,
            state: initial_state,
        }
    }

    fn validate_path(&self) -> ValidationState {
        let mode = self.mode.clone();
        let path_to_validate = self.path_input.get_output();
        match mode {
            ValidationMode::DirectoryContainsFile(target_file) => {
                self.validate_if_file_exists(path_to_validate, target_file.clone())
            }
            ValidationMode::WritableDirectory(file_to_write) => {
                self.validate_writable_directory(path_to_validate, file_to_write)
            }
        }
    }

    fn validate_if_file_exists(
        &self,
        path_to_validate: String,
        target_file: String,
    ) -> ValidationState {
        if path_to_validate.is_empty() {
            return ValidationState::Empty;
        }

        let path_buf = PathBuf::from(path_to_validate);
        if !path_buf.exists() {
            return ValidationState::Invalid("Path does not exist or is incomplete".to_string());
        }

        if !path_buf.is_dir() {
            return ValidationState::Invalid("Path is not a directory".to_string());
        }

        let target_path = path_buf.join(target_file.clone());
        if !target_path.exists() {
            return ValidationState::Invalid(format!(
                "{} does not exist in this directory",
                target_file
            ));
        }

        if !target_path.is_file() {
            return ValidationState::Invalid(format!("{} exists, but is not a file", target_file));
        }

        ValidationState::Ok
    }

    fn validate_writable_directory(
        &self,
        path_to_validate: String,
        file_to_write: String,
    ) -> ValidationState {
        if path_to_validate.is_empty() {
            return ValidationState::Empty;
        }

        let path_buf = PathBuf::from(path_to_validate);
        if !path_buf.exists() {
            return ValidationState::Invalid("Path does not exist or is incomplete".to_string());
        }

        if !path_buf.is_dir() {
            return ValidationState::Invalid("Path is not a directory".to_string());
        }

        let file_path = path_buf.join(file_to_write.clone());
        if file_path.exists() {
            return ValidationState::Warning(format!(
                "Overwriting existing file {}",
                file_to_write
            ));
        }

        ValidationState::Ok
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
    type Output = PathBuf;

    fn get_type(&self) -> WidgetType {
        WidgetType::Input
    }

    fn handle_key(&mut self, key: KeyEvent) {
        self.path_input.handle_key(key);
        self.state = self.validate_path();
    }

    fn output(&self) -> Option<Self::Output> {
        let path_buf = PathBuf::from(self.path_input.get_output());
        let state = self.validate_path();
        match state {
            ValidationState::Empty => None,
            ValidationState::Ok => Some(path_buf),
            ValidationState::Warning(_) => Some(path_buf),
            ValidationState::Invalid(_) => None,
        }
    }

    fn height(&self) -> u16 {
        self.path_input.get_size().height + 2
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        self.path_input.render(frame, area.x, area.y);

        let hint_rect = Rect::new(area.x, area.y + 2, area.width, 1);
        match self.state.clone() {
            ValidationState::Empty => {}
            ValidationState::Ok => {}
            ValidationState::Warning(message) => {
                let paragraph = Paragraph::new(message).style(Style::default().fg(Color::Yellow));
                frame.render_widget(paragraph, hint_rect);
            }
            ValidationState::Invalid(message) => {
                let paragraph = Paragraph::new(message).style(Style::default().fg(Color::Red));
                frame.render_widget(paragraph, hint_rect);
            }
        }
    }
}
