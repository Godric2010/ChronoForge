use crate::widgets::selectable_card_list::card_trait::SelectableCard;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub mod card_trait;
pub mod project_card;
pub mod task_card;
pub mod time_entry_card;

#[derive(Default)]
pub struct SelectableCardList<Card: SelectableCard> {
    pub title: String,
    pub cards: Vec<Card>,
    is_active: bool,
    selected_index: Option<usize>,
    scroll_offset: usize,
    visible_items_count: usize,
    pub item_height: u16,
}

impl<Card: SelectableCard> SelectableCardList<Card> {
    pub fn set_active(&mut self, active: bool, keep_selected_item: bool) {
        self.is_active = active;
        if active {
            self.selected_index = Some(0);
            self.scroll_offset = 0;
        } else if !keep_selected_item {
            self.selected_index = None
        }
    }

    pub fn get_selected_index(&mut self) -> Option<usize> {
        if self.cards.is_empty() {
            return None;
        }

        if let Some(index) = self.selected_index {
            if index >= self.cards.len() {
                self.selected_index = Some(0);
            }
            return Some(index);
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_color = if self.is_active {
            Color::Rgb(255, 125, 0)
        } else {
            Color::Gray
        };

        let list_block = Block::default()
            .title(format!(" {} ", self.title.clone()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let inner = list_block.inner(area);
        frame.render_widget(list_block, area);

        self.visible_items_count = (inner.height / self.item_height) as usize;

        let vertical_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(self.item_height),
            Constraint::Length(1),
        ])
        .split(inner);

        let more_text_top: String = if self.scroll_offset > 0 {
            "↑ more".to_string()
        } else {
            String::new()
        };
        let top_paragraph =
            Paragraph::new(more_text_top).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(top_paragraph, vertical_layout[0]);

        self.render_cards(frame, vertical_layout[1]);

        let more_text_bottom: String =
            if self.scroll_offset + self.visible_items_count < self.cards.len() {
                "↓ more".to_string()
            } else {
                String::new()
            };
        let bottom_paragraph =
            Paragraph::new(more_text_bottom).style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(bottom_paragraph, vertical_layout[2]);
    }

    fn render_cards(&mut self, frame: &mut Frame, area: Rect) {
        for i in 0..self.visible_items_count {
            let item_index = self.scroll_offset + i;

            if item_index >= self.cards.len() {
                break;
            }

            let item_area = Rect {
                x: area.x,
                y: area.y + i as u16 * self.item_height,
                width: area.width,
                height: self.item_height,
            };

            let card = &mut self.cards[item_index];

            if let Some(selected_index) = self.selected_index {
                if selected_index == item_index {
                    card.enable_highlight();
                } else {
                    card.disable_highlight();
                }
            } else {
                card.disable_highlight();
            }

            card.render(frame, item_area);
        }
    }

    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) {
        if self.selected_index.is_none() {
            return;
        }

        let mut selected_index = self.selected_index.unwrap();

        match event.code {
            KeyCode::Down => {
                if selected_index >= self.cards.len() - 1 {
                    return;
                }
                selected_index = selected_index + 1;
                if selected_index >= self.scroll_offset + self.visible_items_count {
                    self.scroll_offset += 1;
                }
                self.selected_index = Some(selected_index);
            }
            KeyCode::Up => {
                if self.cards.is_empty() {
                    return;
                }
                if selected_index == 0 {
                    return;
                }

                selected_index = selected_index - 1;
                if selected_index < self.scroll_offset {
                    self.scroll_offset -= 1;
                }

                self.selected_index = Some(selected_index);
            }
            _ => return,
        }
    }
}
