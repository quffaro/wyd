use crate::app::{
    structs::windows::{BaseWindow, Mode, Popup},
    App,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

pub fn handle_search(key_event: KeyEvent, app: &mut App) {
    match key_event {
        KeyEvent {
            code: KeyCode::Esc | KeyCode::Char('q'),
            ..
        } => {
            app.running = false;
        }
        KeyEvent {
            code: KeyCode::Char('c') | KeyCode::Char('C'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            app.running = false;
        }
        KeyEvent {
            code: KeyCode::Right,
            ..
        } => app.cycle_focus_next(),
        KeyEvent {
            code: KeyCode::Left,
            ..
        } => app.cycle_focus_previous(),
        event => {
            app.input.handle_event(&Event::Key(event));
        }
    }
}

pub fn handle_base(key_event: KeyEvent, app: &mut App) {
    match app.window.mode {
        Mode::Normal => {
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
                KeyCode::Char(';') | KeyCode::Right => app.cycle_focus_next(),
                KeyCode::Char('j') | KeyCode::Left => app.cycle_focus_previous(),
                KeyCode::Tab => {
                    if key_event.modifiers == KeyModifiers::SHIFT {
                        app.cycle_focus_previous()
                    } else {
                        app.cycle_focus_next()
                    }
                }
                KeyCode::Char('l') | KeyCode::Up => app.previous(),
                KeyCode::Char('k') | KeyCode::Down => app.next(),
                // Other handlers you could add here.
                KeyCode::Char('/') => app.to_search(),
                KeyCode::Enter | KeyCode::Char('x') => app.toggle(),
                KeyCode::Char('A') => app.add_project_in_dir(true),
                KeyCode::Char('y') => app.yank(),
                KeyCode::Char('c') => app.cycle_priority(),
                // popup
                KeyCode::Char('T') => app.popup(Popup::ReadTodo, None),
                KeyCode::Char('a') => app.popup(Popup::NewProject, Some(Mode::Insert)),
                KeyCode::Char('r') => app.popup(Popup::EditCat, Some(Mode::Insert)),
                KeyCode::Char('R') => app.popup(Popup::NewCat, None),
                KeyCode::Char('t') => app.popup(Popup::AddTodo, Some(Mode::Insert)),
                KeyCode::Char('e') => app.popup(Popup::EditDesc, Some(Mode::Insert)),
                KeyCode::Char('G') => app.popup(Popup::SearchGitConfigs, Some(Mode::Normal)),
                KeyCode::Char('d') => app.popup(Popup::DeleteProject, None),
                KeyCode::Char('h') => app.popup(Popup::Help, None),
                _ => {}
            }
        }
        Mode::Insert => match key_event.code {
            KeyCode::Esc => app.window.to_normal(),
            KeyCode::Char('l') | KeyCode::Up => app.previous(),
            KeyCode::Char('k') | KeyCode::Down => app.next(),
            KeyCode::Char('p') => app.paste(),
            _ => {}
        },
    }
}
