use super::{FilteredListItems, ListState};
use crate::sql::todo::read_todo;
use rusqlite::Connection;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: u8,
    pub parent_id: u8,
    pub project_id: u8,
    pub todo: String,
    pub is_complete: bool,
    pub priority: String,
}

impl FilteredListItems<Todo> {
    pub fn load(conn: &Connection) -> FilteredListItems<Todo> {
        // TODO replace by Self?
        FilteredListItems {
            items: read_todo(conn).expect("AA"),
            filtered: vec![],
            state: ListState::default(),
        }
    }
    // TODO this is an imperfect one...
    pub fn sort_by_complete(&mut self) {
        self.filtered
            .sort_by(|a, b| a.is_complete.cmp(&b.is_complete))
    }
    // TODO can this be a method for ListNavigate?
    // pub fn toggle(&mut self, conn: &Connection) {
    //     let selected = self.state.selected().unwrap();
    //     for i in 0..self.filtered.len() {
    //         if i == selected {
    //             self.filtered[i].is_complete = !self.filtered[i].is_complete;
    //             update_todo(conn, &self.filtered[i]).expect("msg");
    //         } else {
    //             continue;
    //         }
    //     }
    //     self.sort_by_complete()
    // }
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
    }
}
