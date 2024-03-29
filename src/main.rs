use ini::Ini;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rusqlite::Connection;
use std::str::FromStr;
use std::{io, thread};
use wyd::app::{structs::config::WydColor, App, AppResult};
use wyd::event::{Event, EventHandler};
use wyd::handler::{handle_key_events, handle_mouse_events};
use wyd::tui::Tui;
use wyd::{home_path, PATH_CONFIG, PATH_DB};

fn main() -> AppResult<()> {
    // intiialize db
    let conn = Connection::open(home_path(PATH_DB)).unwrap();
    wyd::sql::initialize_db(&conn)?;

    // Create an application.
    let mut app = App::load();
    app.default_select();

    //     match Ini::load_from_file(home_path(PATH_CONFIG)) {
    //         Ok(c) => {
    //             let y = wyd::app::structs::config::get_config_color(c);
    //             dbg!(y);
    //             Ok(())
    //         }
    //         Err(_) => Ok(()),
    //     }

    // init
    thread::spawn(|| wyd::sql::tmp_config::init_tmp_git_config());

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
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
            Event::RequestComplete => app.finish_github_request(),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
