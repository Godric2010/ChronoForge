pub mod active_timer;
pub mod dialog_widgets;
pub mod elements;
pub mod selectable_card_list;
pub mod tab_widget;

#[cfg(test)]
pub mod test_helper {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    pub fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[cfg(test)]
    pub fn char_key(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
    }
}
