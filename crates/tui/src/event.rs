use crossterm::event::Event;
use std::time::Duration;

pub enum TuiEvent {
    Tick,
    Input(Event),
}
pub fn read_event(timeout: Duration) -> anyhow::Result<TuiEvent> {
    if !crossterm::event::poll(timeout)? {
        return Ok(TuiEvent::Tick);
    }

    loop {
        let event = crossterm::event::read()?;
        match event {
            Event::Key(key_event) => {
                if key_event.is_press() {
                    return Ok(TuiEvent::Input(event));
                }

                // Ignore Release / Repeat events
                if !crossterm::event::poll(Duration::from_millis(5))? {
                    return Ok(TuiEvent::Tick);
                }
            }
            other => return Ok(TuiEvent::Input(other)),
        }
    }
}
