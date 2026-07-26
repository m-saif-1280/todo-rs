use todo_rs::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    if let Err(e) = app.load_tasks() {
        ratatui::restore();
        return Err(e);
    }

    while app.is_running() {
        app.draw();
        app.handle_event()?;
    }

    Ok(())
}
