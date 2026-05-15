use crate::app_action::AppAction;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct ProjectOverviewScreen{
    
}

impl ProjectOverviewScreen {
    pub fn new() -> Self {
       Self{
           
       } 
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect){
        
    }
    
    pub fn handle_event(&mut self, event: Event) -> Option<AppAction> {
        match event {
            Event::Key(key_event)=> match key_event.code {
                KeyCode::Esc => Some(AppAction::Quit),
                _ => None
            },
            _ => None
        }
    }
}