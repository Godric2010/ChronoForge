use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct InputMap<A> {
    key_bindings: Vec<KeyBinding<A>>,
    name: String,
}

impl<A: Copy> InputMap<A> {
    pub fn new(name: &str, key_bindings: Vec<KeyBinding<A>>) -> InputMap<A> {
        Self {
            key_bindings,
            name: String::from(name),
        }
    }

    pub fn find_action(&self, key_event: KeyEvent) -> Option<A> {
        self.key_bindings
            .iter()
            .find(|binding| binding.matches(key_event))
            .map(|binding| binding.action)
    }

    fn get_input_map_helper(&self) -> InputMapHelpContext {
        let key_bindings_helper = self
            .key_bindings
            .iter()
            .map(|kb| kb.get_help_context())
            .collect::<Vec<_>>();
        InputMapHelpContext::new(self.name.clone(), key_bindings_helper)
    }

    pub fn render_help_texts(&self, frame: &mut Frame, rect: Rect) {
        let bindings_len = self.key_bindings.len();
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(bindings_len as u16),
        ])
        .split(rect);

        let heading = Paragraph::new(self.name.clone())
            .style(Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED));
        frame.render_widget(heading, vertical[0]);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(15), Constraint::Percentage(85)])
                .split(vertical[1]);

        let name_column_rect = horizontal[0];
        let desc_column_rect = horizontal[1];

        for (i, key_binding) in self.key_bindings.iter().enumerate() {
            let key_name_paragraph = Paragraph::new(key_binding.key_name.clone());
            let key_description_paragraph = Paragraph::new(key_binding.key_description.clone());

            let name_rect = Rect::new(
                name_column_rect.x,
                name_column_rect.y + i as u16,
                name_column_rect.width,
                1,
            );
            let desc_rect = Rect::new(
                desc_column_rect.x,
                desc_column_rect.y + i as u16,
                desc_column_rect.width,
                1,
            );

            frame.render_widget(key_name_paragraph, name_rect);
            frame.render_widget(key_description_paragraph, desc_rect);
        }
    }
}

impl<A: Copy> HelpProvider for InputMap<A> {
    fn append_footer_help(&self, output: &mut Vec<KeyBindingHelpContext>) {
        output.extend(self.get_input_map_helper().get_footer_helper())
    }

    fn append_general_help(&self, output: &mut Vec<InputMapHelpContext>) {
        output.push(self.get_input_map_helper().clone())
    }
}
