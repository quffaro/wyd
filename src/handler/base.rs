use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, structs::{windows::{Popup, Mode}}};

pub fn handle_base(key_event: KeyEvent, app: &mut App) {
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
        KeyCode::Enter => app.toggle(),
        KeyCode::Right => app.cycle_focus_next(),
        KeyCode::Left => app.cycle_focus_previous(),
        KeyCode::Up => app.previous(),
        KeyCode::Down => app.next(),
        // Other handlers you could add here.
        KeyCode::Char('a') => app.add_project_in_dir(true),
        KeyCode::Char('t') => app.popup(Popup::AddTodo, Some(Mode::Insert)),
        KeyCode::Char('y') => app.popup(Popup::EditDesc,  Some(Mode::Insert)),
        KeyCode::Char('p') => app.popup(Popup::SearchGitConfigs, Some(Mode::Normal)),
        _ => {}
    }
}