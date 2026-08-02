pub mod error_widget;
mod list_widget;
mod path_widget;
mod project_edit_widget;
mod task_edit_widget;
mod time_entry_widget;
mod time_widget;
mod yes_no_widget;

use crate::input::HelpProvider;
use crossterm::event::KeyEvent;
pub use list_widget::{ListItem, ListWidget};
pub use path_widget::*;
pub use project_edit_widget::*;
use ratatui::layout::Rect;
use ratatui::Frame;
pub use task_edit_widget::*;
pub use time_entry_widget::TimeEntryWidget;
pub use time_widget::TimeWidget;
pub use yes_no_widget::YesNoWidget;

pub enum WidgetType {
    Input,
    Error,
}

pub trait DialogWidget: HelpProvider {
    type Output;

    fn get_type(&self) -> WidgetType;

    fn handle_key(&mut self, key: KeyEvent);
    fn output(&self) -> Option<Self::Output>;
    fn height(&self) -> u16;
    fn render(&self, frame: &mut Frame, area: Rect);
}
