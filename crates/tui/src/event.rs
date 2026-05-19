use std::time::Duration;
pub enum TuiEvent {
    Tick,
    Input(crossterm::event::Event),
}
pub fn read_event(timeout: Duration) -> anyhow::Result<TuiEvent> {
    if crossterm::event::poll(timeout)? {
        Ok(TuiEvent::Input(crossterm::event::read()?))
    } else {
        Ok(TuiEvent::Tick)
    }
}
