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
    is_active: bool,
    selected_index: Option<usize>,
    pub item_height: u16,
}

impl<Card: SelectableCard> SelectableCardList<Card> {
    pub fn set_active(&mut self, active: bool, keep_selected_item: bool) {
        self.is_active = active;
        if active {
            self.selected_index = Some(0);
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
                selected_index = selected_index + 1;
                if selected_index >= self.cards.len() {
                    selected_index = 0;
                }
                self.selected_index = Some(selected_index);
            }
            KeyCode::Up => {
                if self.cards.is_empty() {
                    return;
                }
                if selected_index == 0 {
                    self.selected_index = Some(self.cards.len() - 1);
                    return;
                }
                self.selected_index = Some(selected_index - 1);
            }
            _ => return,
        }
    }
}
