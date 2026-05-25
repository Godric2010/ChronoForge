mod list_widget;
mod text_input_widget;
mod time_entry_widget;
mod yes_no_widget;

pub use list_widget::{ListItem, ListWidget};
pub use text_input_widget::{TextInputMode, TextInputWidget};
pub use time_entry_widget::TimeEntryWidget;
pub use yes_no_widget::YesNoWidget;

use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub trait DialogWidget {
    type Output;

    fn handle_key(&mut self, key: KeyEvent);
    fn output(&self) -> Self::Output;
    fn height(&self) -> u16;
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
