use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rusqlite::Connection;
use std::io;
use wyd::app::{App, AppResult};
use wyd::event::{Event, EventHandler};
use wyd::handler::handle_key_events;
use wyd::sql::initialize_db;
use wyd::tui::Tui;
use wyd::{home_path, PATH_DB};

fn main() -> AppResult<()> {
    // intiialize db
    let conn = Connection::open(home_path(PATH_DB)).unwrap();
    initialize_db(&conn)?;
    // Create an application.
    let mut app = App::load();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
