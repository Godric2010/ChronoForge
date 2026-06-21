use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

#[derive(Copy, Clone)]
enum HelpDialogActions {
    Close,
    ScrollUp,
    ScrollDown,
}

pub struct HelpDialog {
    title: String,
    input_help_rows: Vec<HelpTextRow>,
    input_map: InputMap<HelpDialogActions>,
    scroll_offset: u16,
    visible_count: u16,
    draw_rect: Rect,
    footer_text: String,
}

impl HelpDialog {
    pub fn new(input_helper: Vec<InputMapHelpContext>, draw_rect: Rect) -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Esc,
                key_modifier: KeyModifiers::empty(),
                key_name: "Esc".to_string(),
                key_description: "Close".to_string(),
                action: HelpDialogActions::Close,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Up,
                key_modifier: KeyModifiers::empty(),
                key_name: "↑".to_string(),
                key_description: "Scroll up".to_string(),
                action: HelpDialogActions::ScrollUp,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Down,
                key_modifier: KeyModifiers::empty(),
                key_name: "↓".to_string(),
                key_description: "Scroll down".to_string(),
                action: HelpDialogActions::ScrollDown,
                display_in_footer: true,
            },
        ];
        let input_map = InputMap::new("Help Dialog", key_bindings);
        let footer_text = KeyBindingHelpContext::build_single_line(input_map.footer_help());

        let mut input_help_rows = Vec::new();
        input_helper.iter().for_each(|item| {
            input_help_rows.push(HelpTextRow::new(
                item.get_input_map_name().to_string(),
                "".to_string(),
                true,
            ));

            item.get_all_helper().iter().for_each(|item| {
                input_help_rows.push(HelpTextRow::new(
                    item.key_name.clone(),
                    item.description.clone(),
                    false,
                ))
            })
        });

        Self {
            title: "Input Help".to_string(),
            input_help_rows,
            input_map,
            draw_rect,
            scroll_offset: 0,
            visible_count: 30, //(draw_rect.height - 2).min(input_helper.len() as u16),
            footer_text: footer_text.to_string(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        let block = Block::new().title(self.title.clone()).borders(Borders::ALL);
        let inner = block.inner(dialog_draw_rect);
        frame.render_widget(block, dialog_draw_rect);

        let height = inner.height - 2;
        let displayable_items = self.input_help_rows.len().min(height as usize);

        if self.scroll_offset > 0 {
            let more_prev_paragraph = Paragraph::new("↑ more");
            frame.render_widget(
                more_prev_paragraph,
                Rect::new(inner.x, inner.y, inner.width, 1),
            );
        }

        let mut y_offset = 0;
        for display_row in &self.input_help_rows {
            display_row.render(frame, inner.x, inner.y + y_offset, inner.width);
            y_offset += display_row.height;
        }

        // for row_idx in 0..displayable_items {
        //     let idx = row_idx + self.scroll_offset as usize;
        //     if idx >= self.input_help_rows.len() {
        //         break;
        //     }
        //     self.input_help_rows[idx].render(frame, inner.x, inner.y + row_idx as u16, inner.width);
        // }

        if self.scroll_offset as usize + displayable_items < self.input_help_rows.len() {
            let more_next_paragraph = Paragraph::new("↓ more");
            frame.render_widget(
                more_next_paragraph,
                Rect::new(inner.x, inner.y + inner.height, inner.width, 1),
            );
        }
    }
    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let vertical_chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

        let dialog_row = vertical_chunks[1];

        let horizontal_chunks = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Percentage(30),
            Constraint::Min(0),
        ])
        .split(dialog_row);

        horizontal_chunks[1]
    }
    pub fn handle_key(&mut self, event: KeyEvent) -> bool {
        let action = self.input_map.find_action(event);
        if let Some(action) = action {
            match action {
                HelpDialogActions::Close => true,
                HelpDialogActions::ScrollUp => {
                    if self.scroll_offset == 0 {
                        self.scroll_offset = 0;
                    } else {
                        self.scroll_offset -= 1;
                    }
                    false
                }
                HelpDialogActions::ScrollDown => {
                    if self.scroll_offset + self.visible_count < self.input_help_rows.len() as u16 {
                        false
                    } else {
                        self.scroll_offset += 1;
                        false
                    }
                }
            }
        } else {
            false
        }
    }
}

struct HelpTextRow {
    name: String,
    description: String,
    is_headline: bool,
    height: u16,
}

impl HelpTextRow {
    pub fn new(name: String, description: String, is_headline: bool) -> Self {
        Self {
            name,
            description,
            is_headline,
            height: if is_headline { 2 } else { 1 },
        }
    }

    pub fn render(&self, frame: &mut Frame, x_position: u16, y_position: u16, width: u16) {
        let rect = Rect::new(x_position + 1, y_position, width - 2, self.height);
        if self.is_headline {
            self.render_headline(frame, rect)
        } else {
            self.render_key_help(frame, rect)
        }
    }

    fn render_headline(&self, frame: &mut Frame, rect: Rect) {
        let paragraph = Paragraph::new(self.name.clone())
            .style(Style::new().add_modifier(Modifier::BOLD | Modifier::UNDERLINED));
        frame.render_widget(paragraph, Rect::new(rect.x, rect.y + 1, rect.width, 1));
    }

    fn render_key_help(&self, frame: &mut Frame, rect: Rect) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(rect);
        let key_paragraph = Paragraph::new(self.name.clone());
        let desc_paragraph = Paragraph::new(self.description.clone());

        frame.render_widget(key_paragraph, horizontal[0]);
        frame.render_widget(desc_paragraph, horizontal[1]);
    }
}
