// use crate::app::structs;
use super::{ListNav, TableItems, TableState};
use crate::sql::project::{read_project, update_project_status};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use rusqlite::Connection;
use std::{env, fmt};
use strum_macros::EnumString;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u8,
    pub path: String,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub status: ProjectStatus,
    pub is_git: bool,
    pub owner: String,
    pub repo: String,
    pub last_commit: String,
}

impl Project {
    pub fn load(conn: &Connection) -> Vec<Project> {
        read_project(conn).expect("READ PROJECT ERROR")
        // vec![]
    }
    pub fn new_in_pwd() -> Project {
        let current_dir = env::current_dir().unwrap().display().to_string();
        let name = current_dir.clone();
        Project {
            id: 0,
            path: current_dir.clone(),
            name: name,
            desc: "".to_owned(),
            category: "Unknown".to_owned(),
            status: ProjectStatus::Unstable,
            is_git: false,
            owner: "".to_owned(), //TODO
            repo: current_dir.clone(),
            last_commit: "N/A".to_owned(),
        }
    }
    // TODO this is
    pub fn cycle_status(&mut self, conn: &Connection) {
        self.status = match self.status {
            ProjectStatus::Stable => ProjectStatus::Unstable,
            ProjectStatus::Unstable => ProjectStatus::Stable,
        };
        // TODO we need to write this
        update_project_status(conn, self);
    }
}

impl TableItems<Project> {
    pub fn load(conn: &Connection) -> TableItems<Project> {
        TableItems {
            items: Project::load(conn),
            state: TableState::default(),
        }
    }
    // pub fn reload(mut self, conn: Option<Connection>, idx: Option<u8>) {
    //     self = TableItems::<Project>::load(conn);
    //     match idx {
    //         Some(i) => self.select_state(i),
    //         None => self.select_state(0),
    //     }

    // }
    pub fn current(&self) -> Option<&Project> {
        match self.get_state_selected() {
            Some(idx) => self.items.iter().nth(idx),
            None => None,
        }
    }
    pub fn current_state(&self) -> (Option<usize>, Option<&Project>) {
        match self.get_state_selected() {
            Some(idx) => (Some(idx), self.items.iter().nth(idx)),
            None => (None, None),
        }
    }
    pub fn toggle(&mut self, conn: &Connection) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].cycle_status(conn);
            } else {
                continue;
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, EnumString)]
pub enum ProjectStatus {
    Stable,
    Unstable,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
// TODO refactor to include-sql
impl FromSql for ProjectStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse::<ProjectStatus>()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
