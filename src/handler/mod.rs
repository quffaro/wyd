// use std::thread::__FastLocalKeyInner;

use crate::app::structs::windows::{Popup, Window};
use crate::app::{App, AppResult};
use crossterm::event::KeyEvent;

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
        Window {
            popup: Popup::Help, ..
        } => popup::handle_popup_help(key_event, app),
        Window {
            popup: Popup::EditCat, .. 
        } => popup::handle_popup_edit_cat(key_event, app),
        Window {
            popup: Popup::NewCat, .. 
        } => popup::handle_popup_edit_cat(key_event, app),
        Window {
            popup: Popup::Config, ..
        } => popup::handle_popup_wyd_config(key_event, app),
        _ => base::handle_base(key_event, app),
    }

    Ok(())
}
