use super::{ListNav, ListState, PlainListItems};
use crate::{
    app::structs::projects::Project,
    sql::{category::read_category, project::update_project_category},
};
use rusqlite::Connection;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub fn load(conn: &Connection) -> Vec<Category> {
        read_category(conn).expect("READ CATEGORY ERROR")
    }
}

// impl FromSql for Category {
//     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
//         value
//             .as_str()?
//             .parse::<String>()
//             .map_err(|e| FromSqlError::Other(Box::new(e)))
//     }
// }

impl PlainListItems<Category> {
    pub fn load(conn: &Connection) -> PlainListItems<Category> {
        PlainListItems {
            items: Category::load(conn),
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
    pub fn toggle(&mut self, conn: &Connection, project: &Project) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                update_project_category(conn, project, &self.items[i].name); // TODO not the best!
            } else {
                continue;
            }
        }
    }
}
