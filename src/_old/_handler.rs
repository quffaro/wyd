use std::thread::__FastLocalKeyInner;

use crate::app::structs::windows::{Mode, Popup, Window};
use crate::app::{App, AppResult};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.window {
        Window { popup: Popup::AddTodo, ..
        } => handle_popup_todo(key_event, app), 
        _ => handle_base(key_event, app),
    }

    Ok(())
}

fn handle_popup_todo(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Insert => {
            match key_event {
                KeyEvent{ code: KeyCode::Esc, .. } => app.window.mode = Mode::Normal,
                event => {app.input.handle_event(&Event::Key(event))}
            }
        }
        Mode::Normal => match key_event {
            KeyEvent{ code: KeyCode::Char('q'), .. } => {
                app.input = Input::default();
                app.window.popup = Popup::None;
            },
            KeyEvent{ code: KeyCode::Char('i'), .. } => app.window.mode = Mode::Insert,
            _ => {}
        }
    }
}

fn handle_base(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.running = false;
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        // Other handlers you could add here.
        KeyCode::Char('a') => app.add_project_in_dir(true),
        KeyCode::Char('t') => app.popup(Popup::AddTodo, Some(Mode::Insert)),
        _ => {}
    }
}