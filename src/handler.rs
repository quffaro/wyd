use crate::app::structs::windows::{Mode, Popup, Window};
use crate::app::{App, AppResult};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.window {
        Window {
            popup: Popup::AddTodo,
            ..
        } => match key_event {
            k => match k.code {
                KeyCode::Char('q') => {
                    app.input = Input::default();
                    app.window.popup = Popup::None;
                }
                _ => {}
            },
            input => {
                // app.input.handle_event(&Event::input);
            }
        },
        _ => match key_event.code {
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
        },
    }

    Ok(())
}
