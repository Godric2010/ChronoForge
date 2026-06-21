#[derive(Clone)]
pub struct KeyBindingHelpContext {
    pub key_name: String,
    pub description: String,
    pub show_in_footer: bool,
}

impl KeyBindingHelpContext {
    pub fn build_single_line(binding_helpers: Vec<KeyBindingHelpContext>) -> String {
        let mut single_line = String::new();
        let length = binding_helpers.len();

        for (idx, helper) in binding_helpers.iter().enumerate() {
            single_line.push_str(format!("[{}] {}", helper.key_name, helper.description).as_str());
            if idx < length - 1 {
                single_line.push_str(" | ");
            }
        }

        single_line
    }
}

#[derive(Clone)]
pub struct InputMapHelpContext {
    input_map_name: String,
    key_bindings: Vec<KeyBindingHelpContext>,
}

impl InputMapHelpContext {
    pub fn new(input_map_name: String, key_bindings: Vec<KeyBindingHelpContext>) -> Self {
        Self {
            input_map_name,
            key_bindings,
        }
    }

    pub fn get_input_map_name(&self) -> &str {
        &self.input_map_name
    }

    pub fn get_footer_helper(&self) -> Vec<KeyBindingHelpContext> {
        let mut footer_helper: Vec<KeyBindingHelpContext> = Vec::new();
        self.key_bindings.iter().for_each(|helper| {
            if helper.show_in_footer {
                footer_helper.push(helper.clone());
            }
        });
        footer_helper
    }

    pub fn get_all_helper(&self) -> Vec<KeyBindingHelpContext> {
        self.key_bindings.clone()
    }
}
