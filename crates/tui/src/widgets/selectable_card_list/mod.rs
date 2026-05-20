use crate::widgets::selectable_card_list::card_trait::SelectableCard;
use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub mod card_trait;
pub mod project_card;
pub mod task_card;
pub mod time_entry_card;

#[derive(Default)]
pub struct SelectableCardList<Card: SelectableCard> {
    pub title: String,
    pub cards: Vec<Card>,
    pub highlight: bool,
    selected_index: usize,
    pub item_height: u16,
}

impl<Card: SelectableCard> SelectableCardList<Card> {
    pub fn get_selected_index(&self) -> Option<usize> {
        if self.cards.is_empty() {
            return None;
        }
        Some(self.selected_index)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_color = if self.highlight {
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

        let visible_count = inner.height / self.item_height;

        for i in 0..visible_count {
            let item_index = i as usize;

            if item_index >= self.cards.len() {
                break;
            }

            let item_area = Rect {
                x: inner.x,
                y: inner.y + i * self.item_height,
                width: inner.width,
                height: self.item_height,
            };

            let card = &mut self.cards[item_index];

            if item_index == self.selected_index {
                card.enable_highlight();
            } else {
                card.disable_highlight();
            }

            card.render(frame, item_area);
        }
    }
    pub fn handle_event(&mut self, event: &crossterm::event::KeyEvent) {
        match event.code {
            KeyCode::Down => {
                self.selected_index = self.selected_index + 1;
                if self.selected_index >= self.cards.len() {
                    self.selected_index = 0;
                }
            }
            KeyCode::Up => {
                if self.cards.is_empty() {
                    return;
                }
                if self.selected_index == 0 {
                    self.selected_index = self.cards.len() - 1;
                    return;
                }
                self.selected_index = self.selected_index - 1;
            }
            _ => return,
        }
    }
}
