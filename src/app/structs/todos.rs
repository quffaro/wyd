use super::{ListItems, ListNav, ListState, TableState};
// use crate::json::todo::read_todo;
use derive_builder::Builder;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Builder, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Todo {
    pub id: u8,
    pub project_id: u8,
    pub parent_id: u8,
    pub depth: u8,
    pub todo: String,
    pub is_complete: bool,
    pub priority: u8,
}

impl Todo {
    pub fn change_depth(&mut self, inc: i8) {
        self.depth = std::cmp::max(self.depth as i8 + inc, 0) as u8
    }
}
// impl Vec<Todo> {
//     pub fn add_todo(&mut self, todo: Todo) {
//         todo.id = self.id.max() + 1;
//         self.push(todo)
//     }
// }

impl ListItems<Todo> {
    pub fn load() -> ListItems<Todo> {
        // TODO replace by Self?
        ListItems {
            items: vec![],
            // read_todo().expect("AA"),
            state: ListState::default(),
        }
    }
    pub fn current(&self) -> Option<&Todo> {
        match self.get_state_selected() {
            Some(idx) => self.items.iter().nth(idx),
            None => None,
        }
    }
    pub fn current_state(&self) -> (Option<usize>, Option<&Todo>) {
        match self.get_state_selected() {
            Some(idx) => (Some(idx), self.items.iter().nth(idx)),
            None => (None, None),
        }
    }
    // TODO this is an imperfect one...
    pub fn sort_by_complete(&mut self) {
        self.items.sort_by(|a, b| a.is_complete.cmp(&b.is_complete))
    }
    // TODO can this be a method for ListNavigate?
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        // TODO fix
        // for i in 0..self.len() {
        //     if i == selected {
        //         self[i].is_complete = !self[i].is_complete;
        //         // crate::sql::todo::update_todo(&self.filtered[i]).expect("msg");
        //     } else {
        //         continue;
        //     }
        // }
        self.sort_by_complete()
    }
    pub fn select_filter_state(&mut self, idx: Option<usize>, project_id: u8) {
        self.state.select(idx);
        // self.filter_from_projects(project_id);
    }
    pub fn filter_from_projects(&mut self, project_id: u8) {
        let items = self.items.clone();
        self.items = items
            .into_iter()
            .filter(|t| t.project_id == project_id)
            .collect();

        self.sort_by_complete()
    }
}
