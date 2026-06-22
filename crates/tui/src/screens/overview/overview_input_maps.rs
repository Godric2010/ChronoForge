use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::screens::overview::mode::{
    OverviewGeneralActions, ProjectsModeActions, TasksModeActions, TimeEntriesModeActions,
};
use crossterm::event::{KeyCode, KeyModifiers};

pub fn create_general_input_map() -> InputMap<OverviewGeneralActions> {
    let key_bindings = vec![
        KeyBinding {
            key_code: KeyCode::Char('c'),
            key_modifier: KeyModifiers::CONTROL,
            key_name: "CTRL + C".to_string(),
            key_description: "Quit".to_string(),
            action: OverviewGeneralActions::Quit,
            display_in_footer: true,
        },
        KeyBinding {
            key_code: KeyCode::Right,
            key_modifier: KeyModifiers::empty(),
            key_name: "→".to_string(),
            key_description: "Next mode".to_string(),
            action: OverviewGeneralActions::NextMode,
            display_in_footer: true,
        },
        KeyBinding {
            key_code: KeyCode::Left,
            key_modifier: KeyModifiers::empty(),
            key_name: "←".to_string(),
            key_description: "Prev mode".to_string(),
            action: OverviewGeneralActions::PrevMode,
            display_in_footer: true,
        },
        KeyBinding {
            key_code: KeyCode::Char('s'),
            key_modifier: KeyModifiers::empty(),
            key_name: "s".to_string(),
            key_description: "Start/Stop timer for task".to_string(),
            action: OverviewGeneralActions::ToggleTimer,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('?'),
            key_modifier: KeyModifiers::empty(),
            key_name: "?".to_string(),
            key_description: "Help".to_string(),
            action: OverviewGeneralActions::Help,
            display_in_footer: true,
        },
    ];
    InputMap::new("Overview General Actions", key_bindings)
}

pub fn create_project_mode_input_map() -> InputMap<ProjectsModeActions> {
    let key_bindings = vec![
        KeyBinding {
            key_code: KeyCode::Char('n'),
            key_modifier: KeyModifiers::empty(),
            key_name: "n".to_string(),
            key_description: "Create new project".to_string(),
            action: ProjectsModeActions::New,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('e'),
            key_modifier: KeyModifiers::empty(),
            key_name: "e".to_string(),
            key_description: "Edit project".to_string(),
            action: ProjectsModeActions::Edit,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('d'),
            key_modifier: KeyModifiers::empty(),
            key_name: "d".to_string(),
            key_description: "Delete project".to_string(),
            action: ProjectsModeActions::Delete,
            display_in_footer: false,
        },
    ];
    InputMap::new("Project Mode", key_bindings)
}
pub fn create_task_mode_input_map() -> InputMap<TasksModeActions> {
    let key_bindings = vec![
        KeyBinding {
            key_code: KeyCode::Char('n'),
            key_modifier: KeyModifiers::empty(),
            key_name: "n".to_string(),
            key_description: "Create new task".to_string(),
            action: TasksModeActions::New,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('e'),
            key_modifier: KeyModifiers::empty(),
            key_name: "e".to_string(),
            key_description: "Edit task".to_string(),
            action: TasksModeActions::Edit,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('d'),
            key_modifier: KeyModifiers::empty(),
            key_name: "d".to_string(),
            key_description: "Delete task".to_string(),
            action: TasksModeActions::Delete,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('a'),
            key_modifier: KeyModifiers::empty(),
            key_name: "a".to_string(),
            key_description: "Assign task to project".to_string(),
            action: TasksModeActions::AssignToProject,
            display_in_footer: false,
        },
    ];
    InputMap::new("Project Mode", key_bindings)
}
pub fn create_time_entry_mode_input_map() -> InputMap<TimeEntriesModeActions> {
    let key_bindings = vec![
        KeyBinding {
            key_code: KeyCode::Char('n'),
            key_modifier: KeyModifiers::empty(),
            key_name: "n".to_string(),
            key_description: "Create new time entry".to_string(),
            action: TimeEntriesModeActions::New,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('e'),
            key_modifier: KeyModifiers::empty(),
            key_name: "e".to_string(),
            key_description: "Edit time entry".to_string(),
            action: TimeEntriesModeActions::Edit,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('d'),
            key_modifier: KeyModifiers::empty(),
            key_name: "d".to_string(),
            key_description: "Delete time entry".to_string(),
            action: TimeEntriesModeActions::Delete,
            display_in_footer: false,
        },
        KeyBinding {
            key_code: KeyCode::Char('a'),
            key_modifier: KeyModifiers::empty(),
            key_name: "a".to_string(),
            key_description: "Assign time entry to task".to_string(),
            action: TimeEntriesModeActions::AssignToTask,
            display_in_footer: false,
        },
    ];
    InputMap::new("Project Mode", key_bindings)
}
