use todo_rs::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    while app.is_running() {
        app.draw();
        app.handle_event()?;
    }

    Ok(())
}
