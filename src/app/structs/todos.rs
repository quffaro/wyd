use super::{FilteredListItems, ListNav, ListState, TableState};
use crate::json::todo::read_todo;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: u8,
    pub parent_id: u8,
    pub project_id: u8,
    pub todo: String,
    pub is_complete: bool,
    pub priority: u8,
}

impl FilteredListItems<Todo> {
    pub fn load() -> FilteredListItems<Todo> {
        // TODO replace by Self?
        FilteredListItems {
            items: read_todo().expect("AA"),
            filtered: vec![],
            state: TableState::default(),
        }
    }
    pub fn current(&self) -> Option<&Todo> {
        match self.get_state_selected() {
            Some(idx) => self.filtered.iter().nth(idx),
            None => None,
        }
    }
    pub fn current_state(&self) -> (Option<usize>, Option<&Todo>) {
        match self.get_state_selected() {
            Some(idx) => (Some(idx), self.filtered.iter().nth(idx)),
            None => (None, None),
        }
    }
    // TODO this is an imperfect one...
    pub fn sort_by_complete(&mut self) {
        self.filtered
            .sort_by(|a, b| a.is_complete.cmp(&b.is_complete))
    }
    // TODO can this be a method for ListNavigate?
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.filtered.len() {
            if i == selected {
                self.filtered[i].is_complete = !self.filtered[i].is_complete;
                crate::sql::todo::update_todo(&self.filtered[i]).expect("msg");
            } else {
                continue;
            }
        }
        self.sort_by_complete()
    }
    pub fn select_filter_state(&mut self, idx: Option<usize>, project_id: u8) {
        self.state.select(idx);
        self.filter_from_projects(project_id);
    }
    pub fn filter_from_projects(&mut self, project_id: u8) {
        let items = self.items.clone();
        self.filtered = items
            .into_iter()
            .filter(|t| t.project_id == project_id)
            .collect();

        self.sort_by_complete()
    }
}
