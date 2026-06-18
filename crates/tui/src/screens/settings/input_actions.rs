use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Clone, Copy)]
pub enum SettingsActions {
    Quit,
    NextItem,
    PrevItem,
    NextSection,
    PrevSection,
    Select,
}

pub fn create_settings_input_map() -> InputMap<SettingsActions> {
    let key_bindings = vec![
        KeyBinding {
            key_code: KeyCode::Down,
            key_modifier: KeyModifiers::empty(),
            key_name: "Down".to_string(),
            key_description: "Next".to_string(),
            action: SettingsActions::NextItem,
            display_in_footer: true,
        },
        KeyBinding {
            key_code: KeyCode::Up,
            key_modifier: KeyModifiers::empty(),
            key_name: "Up".to_string(),
            key_description: "Prev".to_string(),
            action: SettingsActions::PrevItem,
            display_in_footer: true,
        },
        KeyBinding {
            key_code: KeyCode::Tab,
            key_modifier: KeyModifiers::empty(),
            key_name: "Tab".to_string(),
            key_description: "Next section".to_string(),
            action: SettingsActions::NextSection,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::BackTab,
            key_modifier: KeyModifiers::empty(),
            key_name: "BackTab".to_string(),
            key_description: "Prev section".to_string(),
            action: SettingsActions::PrevSection,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Enter,
            key_modifier: KeyModifiers::empty(),
            key_name: "Enter".to_string(),
            key_description: "Select".to_string(),
            action: SettingsActions::Select,
            display_in_footer: true,
        },
        KeyBinding {
            key_code: KeyCode::Char('c'),
            key_modifier: KeyModifiers::CONTROL,
            key_name: "CTRL + c".to_string(),
            key_description: "Quit".to_string(),
            action: SettingsActions::Quit,
            display_in_footer: true,
        },
    ];
    InputMap::new("Settings Actions", key_bindings)
}
