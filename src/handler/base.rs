use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::{
    structs::windows::{Mode, Popup},
    App,
};

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
        KeyCode::Char('A') => app.add_project_in_dir(true),
        KeyCode::Char('r') => app.popup(Popup::EditCat, Some(Mode::Insert)),
        KeyCode::Char('R') => app.popup(Popup::NewCat, None),
        KeyCode::Char('t') => app.popup(Popup::AddTodo, Some(Mode::Insert)),
        KeyCode::Char('y') => app.popup(Popup::EditDesc, Some(Mode::Insert)),
        KeyCode::Char('p') => {},
        KeyCode::Char('f') => {}, // status
        KeyCode::Char('G') => app.popup(Popup::SearchGitConfigs, Some(Mode::Normal)),
        KeyCode::Char('g') => {},
        KeyCode::Char('h') => app.popup(Popup::Help, None),
        _ => {}
    }
}
