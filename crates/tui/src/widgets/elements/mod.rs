mod checkbox_element;
mod date_edit_element;
mod text_edit_element;
mod time_edit_element;

use crate::input::HelpProvider;
pub use checkbox_element::*;
use crossterm::event::KeyEvent;
pub use date_edit_element::*;
use ratatui::Frame;
pub use text_edit_element::*;
pub use time_edit_element::*;

pub struct ElementSize {
    pub width: u16,
    pub height: u16,
}

pub trait WidgetElement: HelpProvider {
    type Output;

    fn set_active(&mut self, active: bool);
    fn get_size(&self) -> &ElementSize;

    fn render(&self, frame: &mut Frame, pos_x: u16, pos_y: u16);

    fn handle_key(&mut self, key: KeyEvent);

    fn get_output(&self) -> Self::Output;
}
