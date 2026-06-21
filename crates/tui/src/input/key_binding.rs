use crate::input::help_context::KeyBindingHelpContext;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct KeyBinding<A> {
    pub key_code: KeyCode,
    pub key_modifier: KeyModifiers,
    pub key_name: String,
    pub key_description: String,
    pub action: A,
    pub display_in_footer: bool,
}

impl<A> KeyBinding<A> {
    pub fn matches(&self, key_event: KeyEvent) -> bool {
        key_event.code == self.key_code && key_event.modifiers == self.key_modifier
    }

    pub fn get_help_context(&self) -> KeyBindingHelpContext {
        KeyBindingHelpContext {
            key_name: self.key_name.clone(),
            description: self.key_description.clone(),
            show_in_footer: self.display_in_footer,
        }
    }
}
