use crate::input::help_context::{InputMapHelpContext, KeyBindingHelpContext};
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::{symbols, Frame};

#[derive(Copy, Clone)]
enum HelpDialogActions {
    Close,
    ScrollUp,
    ScrollDown,
}

pub struct HelpDialog {
    title: String,
    input_help_rows: Vec<HelpRow>,
    input_map: InputMap<HelpDialogActions>,
    scroll_offset: u16,
    visible_count: u16,
    footer_text: String,
    more_indicator_style: Style,
}

impl HelpDialog {
    pub fn new(input_helper: Vec<InputMapHelpContext>) -> Self {
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
            input_help_rows.push(HelpRow::Heading(item.get_input_map_name().to_string()));

            item.get_all_helper().iter().for_each(|item| {
                input_help_rows.push(HelpRow::Description(
                    item.key_name.clone(),
                    item.description.clone(),
                ))
            });
            input_help_rows.push(HelpRow::Spacer);
        });

        Self {
            title: "Input Help".to_string(),
            input_help_rows,
            input_map,
            scroll_offset: 0,
            visible_count: 30,
            footer_text: footer_text.to_string(),
            more_indicator_style: Style::default()
                .fg(Color::Rgb(255, 125, 0))
                .add_modifier(Modifier::ITALIC),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        let block = Block::new()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White));
        let inner = block.inner(dialog_draw_rect);
        frame.render_widget(block, dialog_draw_rect);

        let inner_chunks = Layout::vertical([
            Constraint::Length(1),                  // spacer
            Constraint::Length(self.visible_count), // help texts
            Constraint::Length(1),                  // spacer
            Constraint::Length(1),                  // separator
            Constraint::Length(1),                  // footer
        ])
        .split(inner);

        if self.scroll_offset > 0 {
            let more_prev_paragraph = Paragraph::new("↑ more").style(self.more_indicator_style);
            frame.render_widget(more_prev_paragraph, inner_chunks[0]);
        }

        self.draw_help_text_rows(frame, inner_chunks[1]);

        let top_row_idx = self.scroll_offset as usize + self.visible_count as usize;
        if top_row_idx < self.input_help_rows.len() {
            let more_next_paragraph = Paragraph::new("↓ more").style(self.more_indicator_style);
            frame.render_widget(more_next_paragraph, inner_chunks[2]);
        }
        self.render_separator(frame, inner_chunks[3]);
        self.render_help_text(frame, inner_chunks[4]);
    }

    fn draw_help_text_rows(&self, frame: &mut Frame, area: Rect) {
        let max_entries = area.height;
        let mut row_idx = 0;
        let mut y_pos = area.y;
        while y_pos < max_entries + area.y {
            if row_idx >= self.input_help_rows.len() {
                break;
            }

            let help_row_idx = row_idx + self.scroll_offset as usize;

            let row_content = &self.input_help_rows[help_row_idx];
            row_content.render(frame, area.x, y_pos, area.width);
            y_pos += 1;
            row_idx += 1;
        }
    }
    fn render_separator(&self, frame: &mut Frame, area: Rect) {
        let separator = symbols::line::HORIZONTAL.repeat(area.width.saturating_sub(2) as usize);
        let separator_widget = Paragraph::new(Line::from(separator));
        let mut rect = area;
        rect.x += 1;
        frame.render_widget(separator_widget, rect);
    }

    fn render_help_text(&self, frame: &mut Frame, area: Rect) {
        let target_area = Rect::new(area.x + 1, area.y, area.width - 1, 1);

        let help_text = self.footer_text.as_str();
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, target_area);
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
                    if self.scroll_offset + self.visible_count >= self.input_help_rows.len() as u16
                    {
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

enum HelpRow {
    Heading(String),
    Description(String, String),
    Spacer,
}

impl HelpRow {
    pub fn render(&self, frame: &mut Frame, x_pos: u16, y_pos: u16, width: u16) {
        let rect = Rect::new(x_pos, y_pos, width, 1);
        match self {
            HelpRow::Heading(heading) => self.render_headline(frame, rect, heading),
            HelpRow::Description(key_name, description) => {
                self.render_help_text(frame, rect, key_name, description);
            }
            HelpRow::Spacer => {}
        }
    }

    fn render_headline(&self, frame: &mut Frame, rect: Rect, heading: &str) {
        let paragraph = Paragraph::new(heading)
            .style(Style::new().add_modifier(Modifier::BOLD | Modifier::UNDERLINED));
        frame.render_widget(paragraph, Rect::new(rect.x, rect.y, rect.width, 1));
    }

    fn render_help_text(&self, frame: &mut Frame, rect: Rect, key_name: &str, description: &str) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(rect);
        let key_paragraph = Paragraph::new(key_name);
        let desc_paragraph = Paragraph::new(description);

        frame.render_widget(key_paragraph, horizontal[0]);
        frame.render_widget(desc_paragraph, horizontal[1]);
    }
}
