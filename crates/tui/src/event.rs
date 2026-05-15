pub fn read_event() -> anyhow::Result<crossterm::event::Event> {
    Ok(crossterm::event::read()?)
}
