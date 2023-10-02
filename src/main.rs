#![allow(unused_imports)]
#![allow(unused_doc_comments)]

#[macro_use]
extern crate derive_builder;

use ini::Ini;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::str::FromStr;
use std::{io, thread};
use wyd::app::{structs::config::WydColor, App, AppResult};
use wyd::event::{Event, EventHandler};
use wyd::handler::{handle_key_events, handle_mouse_events};
use wyd::tui::Tui;
use wyd::{home_path, PATH_CONFIG, PATH_DB};

use wyd::app::structs::projects::Project;
use wyd::json::{read_json, SaveFile};

fn main() -> AppResult<()> {
    // intiialize db
    // wyd::json::initialize_db(&conn)?;
    // TODO retype as SaveFile
    let projects: Vec<Project> = read_json()?;
    // println!("MAIN: {:?}", projects);

    // Create an application.
    let mut app = App::load();
    app.default_select();

    // println!("APP: {:?}", &app.projects.items);
    //     match Ini::load_from_file(home_path(PATH_CONFIG)) {
    //         Ok(c) => {
    //             let y = wyd::app::structs::config::get_config_color(c);
    //             dbg!(y);
    //             Ok(())
    //         }
    //         Err(_) => Ok(()),
    //     }

    // init
    // thread::spawn(|| wyd::json::tmp_config::init_tmp_git_config());

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
