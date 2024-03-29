use crate::app::{
    structs::projects::ProjectBuilder,
    structs::windows::{Mode, Popup},
    App,
};
use crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub fn handle_popup_new_project(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Esc, ..
            } => app.window.to_normal(),
            event => {
                app.input.handle_event(&Event::Key(event));
            }
        },
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => app.write_close_new_project(
                ProjectBuilder::new()
                    .name(app.input.value().to_owned())
                    .build(),
            ),
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            _ => {}
        },
    }
}

pub fn handle_popup_delete_project(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert | Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Tab, ..
            } => app.index = (app.index + 1) % 2,
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => match app.index {
                0 => app.delete_project(),
                _ => {}
            },
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            _ => {}
        },
    }
}

pub fn handle_popup_todo(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Esc, ..
            } => app.window.to_normal(),
            event => {
                app.input.handle_event(&Event::Key(event));
            }
        },
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => app.write_close_new_todo(),
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            _ => {}
        },
    }
}

pub fn handle_popup_desc(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Esc, ..
            } => app.window.to_normal(),
            event => {
                app.input.handle_event(&Event::Key(event));
            }
        },
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => app.write_close_description(),
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            _ => {}
        },
    }
}

pub fn handle_popup_search_configs(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Esc, ..
            } => app.window.to_normal(),
            event => {
                app.input.handle_event(&Event::Key(event));
            }
        },
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => app.toggle(),
            KeyEvent {
                code: KeyCode::Up, ..
            } => app.previous(),
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => app.next(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => app.write_close_gitconfig(),
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            _ => {}
        },
    }
}

pub fn handle_popup_read_todo(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        /* COMMON */
        Mode::Normal | Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            _ => {}
        },
    }
}

pub fn handle_popup_help(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        /* COMMON */
        Mode::Normal | Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('h'),
                ..
            } => app.default_close(),
            _ => {}
        },
    }
}

pub fn handle_popup_wyd_config(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Esc, ..
            } => app.window.to_normal(),
            event => {
                app.input.handle_event(&Event::Key(event));
            }
        },
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => {
                crate::app::structs::config::init_config(crate::app::structs::config::Config {
                    owner: app.input.value().to_owned(),
                    search_folder: crate::home_path(crate::PATH_CONFIG),
                    db: crate::home_path(crate::PATH_DB),
                    color: crate::app::structs::config::ColorConfig::default(),
                });
                app.default_close();
            }
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            _ => {}
        },
    }
}

pub fn handle_popup_edit_cat(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        /* COMMON */
        Mode::Normal | Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            KeyEvent {
                code: KeyCode::Tab, ..
            } => app.window.popup = Popup::NewCat,
            KeyEvent {
                code: KeyCode::Up, ..
            } => app.previous(),
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => app.next(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            }
            | KeyEvent {
                code: KeyCode::Enter,
                ..
            } => app.write_close_edit_category(), /* TODO */
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            _ => {}
        },
    }
}

pub fn handle_popup_new_cat(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Tab, ..
            } => app.window.popup = Popup::EditCat,
            KeyEvent {
                code: KeyCode::Esc, ..
            } => app.window.to_normal(),
            event => {
                app.input.handle_event(&Event::Key(event));
            }
        },
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => app.default_close(),
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => app.toggle(),
            KeyEvent {
                code: KeyCode::Tab, ..
            } => app.window.popup = Popup::EditCat,
            KeyEvent {
                code: KeyCode::Up, ..
            } => app.previous(),
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => app.next(),
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => app.write_close_new_category(), /* TODO */
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => app.window.to_insert(),
            _ => {}
        },
    }
}
