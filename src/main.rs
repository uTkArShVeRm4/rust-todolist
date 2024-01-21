/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use rusqlite::Connection;
use tui::Tui;
use update::update;
type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;
fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();
    let conn = Connection::open("task.db")?;

    app.todo_items = vec![
        String::from("TASK                DEADLINE"),
        String::from("lol"),
        String::from("epic"),
        String::from("wow"),
        String::from("lmaoo"),
    ];
    app.todo_select_state.select(Some(1));
    app.load_database(&conn)?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
