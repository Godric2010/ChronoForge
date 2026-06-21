use crate::input::help_context::KeyBindingHelpContext;

pub mod help_context;
pub mod input_map;
pub mod key_binding;

pub trait HelpProvider {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>);

    fn footer_help(&self) -> Vec<KeyBindingHelpContext> {
        let mut output: Vec<KeyBindingHelpContext> = Vec::new();
        self.append_footer_help(&mut output);
        output
    }
}
