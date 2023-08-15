use super::{ListNav, ListState, PlainListItems};
use crate::{
    app::structs::projects::Project,
    sql::{category::read_category, project::update_project_category},
};
use rusqlite::Connection;
use std::fmt;
use serde::{Serialize, Deserialize}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u8,
    pub name: String,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Category {
    pub fn load() -> Vec<Category> {
        read_category().expect("READ CATEGORY ERROR")
    }
}

impl PlainListItems<Category> {
    pub fn load() -> PlainListItems<Category> {
        PlainListItems {
            items: Category::load(),
            state: ListState::default(),
        }
    }
    // TODO toggle
    pub fn current(&self) -> Option<&Category> {
        match self.get_state_selected() {
            Some(idx) => self.items.iter().nth(idx),
            None => None,
        }
    }
    pub fn toggle(&mut self, project: &Project) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                update_project_category(project, &self.items[i].name); // TODO not the best!
            } else {
                continue;
            }
        }
    }
}
