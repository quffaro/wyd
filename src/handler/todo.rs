use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::app::{App, structs::{windows::{Popup, Mode}, todos::Todo}};


pub fn handle_popup_todo(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => {
            match key_event {
                KeyEvent{ code: KeyCode::Esc, .. } => app.window.to_normal(),
                event => {app.input.handle_event(&Event::Key(event));}
            }
        }
        /* COMMON */
        Mode::Normal => match key_event {
            KeyEvent{ code: KeyCode::Char('q'), .. } => {
                /* TODO close */
                app.default_input();
                app.close_popup();
            },
            KeyEvent{ code: KeyCode::Char('w'), .. } => app.write_close_new_todo(),
            KeyEvent{ code: KeyCode::Char('i'), .. } => app.window.to_insert(),
            _ => {}
        }
    }
}