use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};

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

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>);

    fn general_help(&self) -> Vec<InputMapHelpContext> {
        let mut output: Vec<InputMapHelpContext> = Vec::new();
        self.append_general_help(&mut output);
        output
    }
}
