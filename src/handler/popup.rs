use crate::app::{
    structs::{
        todos::Todo,
        windows::{Mode, Popup, Window},
    },
    App,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

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
            } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            }
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
            } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            }
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
            } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            }
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

pub fn handle_popup_help(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        // Mode::Insert => {
        //     match key_event {
        //         KeyEvent{ code: KeyCode::Esc, .. } => app.window.to_normal(),
        //         event => {app.input.handle_event(&Event::Key(event));}
        //     }
        // }
        /* COMMON */
        Mode::Normal | Mode::Insert => match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('h'),
                ..
            } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            }
            _ => {}
        },
    }
}

pub fn handle_popup_edit_cat(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        // Mode::Insert => {
        //     match key_event {
        //         KeyEvent{ code: KeyCode::Esc, .. } => app.window.to_normal(),
        //         event => {app.input.handle_event(&Event::Key(event));}
        //     }
        // }
        /* COMMON */
        Mode::Normal | Mode::Insert => match key_event {
            // KeyEvent{ code: KeyCode::Esc, .. } => app.window.to_normal(),
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            }
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
            } | KeyEvent {
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
            } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            }
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
