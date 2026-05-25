pub mod text_input;

use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

pub trait DialogWidget {
    type Output;

    fn handle_key(&mut self, key: KeyEvent);
    fn output(&self) -> Self::Output;
    fn height(&self) -> u16;
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
