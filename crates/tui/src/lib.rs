mod app;
mod screens;
mod terminal;
mod widgets;
mod event;
mod app_action;

pub async fn run() -> anyhow::Result<()> {
    let mut terminal = terminal::init_terminal()?;

    let result = app::App::new().run(&mut terminal).await;
    
    terminal::restore_terminal()?;
    
    result
}
