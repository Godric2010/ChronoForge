use ratatui::layout::Rect;
use ratatui::Frame;

pub trait SelectableCard {
    fn enable_highlight(&mut self);
    fn disable_highlight(&mut self);
    fn render(&self, frame: &mut Frame, rect: Rect);
}
