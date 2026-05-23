use std::time::Duration;
use crossterm::event::Event;

pub enum TuiEvent {
    Tick,
    Input(crossterm::event::Event),
}
pub fn read_event(timeout: Duration) -> anyhow::Result<TuiEvent> {

    if !crossterm::event::poll(timeout)? {
        return Ok(TuiEvent::Tick);
    }

    loop {
        let event = crossterm::event::read()?;
        match event{
            Event::Key(key_event) =>{
                if key_event.is_press(){
                    return Ok(TuiEvent::Input(event));
                }

                // Ignore Release / Repeat events
                continue
            },
            other => return Ok(TuiEvent::Input(other))
        }
    }
}
