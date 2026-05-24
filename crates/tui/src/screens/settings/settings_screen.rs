use crate::app_action::AppAction;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct SettingsScreen {}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame, rect: Rect) {}

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<AppAction> {
        None
    }
}
