// TODO todos want date column
// TODO shift-a for git repo
// TODO SeaORM
// TODO Oso
// TODO ui folder
use crate::library::code::{
    home_path, App, BaseWindow, ListNavigate, LoadingState, Mode, PopupWindow, Window,
    WindowStatus, HIGHLIGHT_SYMBOL, IN_HOME_DATABASE, SEARCH_DIRECTORY_PREFIX, SUBPATH_GIT_CONFIG,
    SUB_HOME_FOLDER,
};
use crate::library::config::init_config;
use crate::library::request::request_string;
use crate::library::sql::init_tmp_git_config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs::home_dir;
use rusqlite::Connection;
use std::env::current_dir;
use std::io;
use std::sync::mpsc::{self, TryRecvError};
use tui::text::Text;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
    },
    Frame, Terminal,
};
use tui_textarea::{Input, Key, TextArea};
use tui_tree_widget;
// use tokio::task;
use std::thread::{self, current, sleep};
use std::time;

pub fn viewer() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let conn = Connection::open(home_path(IN_HOME_DATABASE)).unwrap();
    let app = App::load(&conn);
    let _res = run_app(&mut terminal, app);

    // Exit App
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // select
    app.default_select();

    // TODO passing the same connection is unsafe.
    // TODO run if there is a config file
    let (tx, rx) = mpsc::channel();
    match app.config {
        Some(_) => {
            thread::spawn(|| init_tmp_git_config());
            thread::spawn(move || {
                request_string();
                // thread::sleep(time::Duration::from_secs(10));
                tx.send(true).unwrap()
            });
        }
        None => {}
    }

    let mut textarea = TextArea::default();
    loop {
        terminal.draw(|rect| {
            ui(rect, &mut app);
            ui_popup(rect, &mut textarea, &mut app);
        })?;

        // TODO do we
        match rx.try_recv() {
            Err(TryRecvError::Empty) => app.throbber.throb.calc_next(),
            Ok(_) | Err(_) => app.throbber.status = WindowStatus::Loaded,
        }

        // app.on_tick();
        // app.input(&mut textarea);
        // TODO i would like for this to be in its own rule
        match app.window {
            Window {
                popup: PopupWindow::None,
                base: _,
                ..
            } => match crossterm::event::read().expect("MAIN CAPTURE ERROR").into() {
                Input {
                    key: Key::Char('q'),
                    ..
                } => return Ok(()),
                Input {
                    key: Key::Char('h'),
                    ..
                } => app.popup(PopupWindow::Help, Some(Mode::Normal)),
                Input {
                    key: Key::Char('a'),
                    ..
                } => app.add_project_in_dir(false),
                Input {
                    key: Key::Char('A'),
                    ..
                } => app.add_project_in_dir(true),
                Input {
                    key: Key::Char('d'),
                    ..
                } => app.delete_todo(),
                Input {
                    key: Key::Char('e'),
                    ..
                } => app.popup(PopupWindow::EditDesc, Some(Mode::Insert)),
                Input {
                    key: Key::Char('r'),
                    ..
                } => app.popup(PopupWindow::EditCategory, Some(Mode::Normal)),
                Input {
                    key: Key::Char('R'),
                    ..
                } => app.popup(PopupWindow::NewCategory, Some(Mode::Insert)),
                Input {
                    key: Key::Char('t'),
                    ..
                } => app.popup(PopupWindow::AddTodo, Some(Mode::Insert)),
                Input {
                    key: Key::Char('p'),
                    ..
                } => app.popup(PopupWindow::SearchGitConfig, Some(Mode::Normal)),
                Input {
                    key: Key::Char('x'),
                    ..
                } => match app.window {
                    Window {
                        base: BaseWindow::Todo,
                        popup: PopupWindow::None,
                        ..
                    } => app.toggle(),
                    _ => {}
                },
                Input {
                    key: Key::Enter, ..
                } => app.toggle(),
                Input {
                    key: Key::Char(';'),
                    ..
                }
                | Input {
                    key: Key::Right, ..
                } => app.cycle_focus_next(),
                Input {
                    key: Key::Char('j'),
                    ..
                }
                | Input { key: Key::Left, .. } => app.cycle_focus_previous(),
                Input {
                    key: Key::Char('l'),
                    ..
                }
                | Input { key: Key::Up, .. }
                | Input {
                    key: Key::MouseScrollUp,
                    ..
                } => app.previous(),
                Input {
                    key: Key::Char('k'),
                    ..
                }
                | Input { key: Key::Down, .. }
                | Input {
                    key: Key::MouseScrollDown,
                    ..
                } => app.next(),
                _ => {}
            },
            Window {
                ref popup, base: _, ..
            } => match app.window.mode {
                Mode::Insert => app.popup_mode_insert(&mut textarea),
                Mode::Normal => app.popup_mode_normal(&mut textarea, popup.clone()),
            },
            _ => {}
        }
    }
}
