pub mod error_widget;
mod list_widget;
mod project_edit_widget;
mod text_input_widget;
mod time_entry_widget;
mod yes_no_widget;

use crossterm::event::KeyEvent;
pub use list_widget::{ListItem, ListWidget};
pub use project_edit_widget::*;
use ratatui::layout::Rect;
use ratatui::Frame;
pub use text_input_widget::{TextInputMode, TextInputWidget};
pub use time_entry_widget::TimeEntryWidget;
pub use yes_no_widget::YesNoWidget;

pub enum WidgetType {
    Input,
    Error,
}

pub trait DialogWidget {
    type Output;

    fn get_type(&self) -> WidgetType;

    fn get_help_text(&self) -> String;

    fn handle_key(&mut self, key: KeyEvent);
    fn output(&self) -> Self::Output;
    fn height(&self) -> u16;
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
