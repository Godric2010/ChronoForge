use crate::input::help_context::KeyBindingHelpContext;
use crate::input::input_map::InputMap;
use crate::input::key_binding::KeyBinding;
use crate::input::HelpProvider;
use crate::widgets::dialog_widgets::{DialogWidget, WidgetType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::{symbols, Frame};

pub enum DialogResult<T> {
    None,
    Cancelled,
    Confirmed(T),
}

#[derive(Copy, Clone)]
enum DialogActions {
    Confirm,
    Cancel,
    Help,
}

pub struct Dialog<Widget: DialogWidget> {
    title: String,
    widget: Widget,
    height: u16,
    input_map: InputMap<DialogActions>,
    footer_text: String,
}

impl<Widget: DialogWidget> Dialog<Widget> {
    pub fn new(title: &str, widget: Widget) -> Self {
        let key_bindings = vec![
            KeyBinding {
                key_code: KeyCode::Enter,
                key_modifier: KeyModifiers::empty(),
                key_name: "↲".to_string(),
                key_description: "Confirm".to_string(),
                action: DialogActions::Confirm,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Esc,
                key_modifier: KeyModifiers::empty(),
                key_name: "Esc".to_string(),
                key_description: "Cancel".to_string(),
                action: DialogActions::Cancel,
                display_in_footer: true,
            },
            KeyBinding {
                key_code: KeyCode::Char('?'),
                key_modifier: KeyModifiers::empty(),
                key_name: "?".to_string(),
                key_description: "Help".to_string(),
                action: DialogActions::Help,
                display_in_footer: true,
            },
        ];
        let input_map = InputMap::new("Dialog Actions", key_bindings);
        let mut footer = widget.footer_help();
        input_map.append_footer_help(&mut footer);
        let footer_text = KeyBindingHelpContext::build_single_line(footer);

        Self {
            title: String::from(title),
            height: 6 + &widget.height(),
            widget,
            input_map,
            footer_text,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let dialog_draw_rect = self.calculate_draw_rect(area);
        frame.render_widget(Clear, dialog_draw_rect);

        // outer block
        let frame_color = match self.widget.get_type() {
            WidgetType::Input => Color::Rgb(255, 150, 0),
            WidgetType::Error => Color::Red,
        };
        let outer_block = Block::default()
            .title(format!("< {} >", self.title))
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(frame_color));
        frame.render_widget(outer_block, dialog_draw_rect);

        // inner blocks
        let inner_chunks = Layout::vertical([
            Constraint::Length(1),                    // border
            Constraint::Length(1),                    // spacer
            Constraint::Length(self.widget.height()), // widget
            Constraint::Length(1),                    // spacer
            Constraint::Length(1),                    // separator
            Constraint::Length(1),                    // help text
        ])
        .split(dialog_draw_rect);

        let mut widget_rect = inner_chunks[2];
        widget_rect.width -= 2;
        widget_rect.x += 1;

        self.widget.render(frame, widget_rect);
        self.render_separator(frame, inner_chunks[4]);
        self.render_help_text(frame, inner_chunks[5]);
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> DialogResult<Widget::Output> {
        let action = self.input_map.find_action(key_event);
        match action {
            None => {
                self.widget.handle_key(key_event);
                let mut footer = self.input_map.footer_help();
                self.widget.append_footer_help(&mut footer);
                KeyBindingHelpContext::build_single_line(footer);
                DialogResult::None
            }
            Some(action) => match action {
                DialogActions::Confirm => DialogResult::Confirmed(self.widget.output()),
                DialogActions::Cancel => DialogResult::Cancelled,
                DialogActions::Help => DialogResult::None,
            },
        }
    }

    fn calculate_draw_rect(&self, area: Rect) -> Rect {
        let vertical_chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(self.height),
            Constraint::Min(0),
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
}
