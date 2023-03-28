// use std::thread::__FastLocalKeyInner;

use crate::app::structs::windows::{Mode, Popup, Window};
use crate::app::{App, AppResult};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

pub mod base;
pub mod popup;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.window {
        Window {
            popup: Popup::AddTodo,
            ..
        } => popup::handle_popup_todo(key_event, app),
        Window {
            popup: Popup::EditDesc,
            ..
        } => popup::handle_popup_desc(key_event, app),
        Window {
            popup: Popup::SearchGitConfigs,
            .. 
        } => popup::handle_popup_search_configs(key_event, app),
        _ => base::handle_base(key_event, app),
    }

    Ok(())
}
