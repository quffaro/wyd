// use crate::app::structs;
use super::{ListNav, TableItems, TableState};
use crate::app::structs::todos::Todo;
use crate::json::project::{read_project, update_project_status};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{env, fmt};
use strum_macros::EnumString;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: u8, // TODO default 0
    pub sort: u8,
    pub path: String,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub status: ProjectStatus,
    pub is_git: bool,
    pub owner: String,
    pub repo: String,
    pub last_commit: String,
    pub todos: Vec<Todo>,
}

impl Project {
    pub fn builder() -> ProjectBuilder {
        ProjectBuilder::default()
    }
    pub fn load() -> Vec<Project> {
        let projects = read_project().expect("READ PROJECT ERROR");

        projects.into_iter().sorted_by_key(|x| x.sort).collect()
    }
    // TODO
    pub fn new_in_pwd() -> Project {
        let current_dir = env::current_dir().unwrap().display().to_string();
        let name = current_dir.clone();
        Project {
            id: 0,
            sort: 0,
            path: current_dir.clone(),
            name: name,
            desc: "".to_owned(),
            category: "Unknown".to_owned(),
            status: ProjectStatus::Unstable,
            is_git: false,
            owner: "".to_owned(), //TODO
            repo: current_dir.clone(),
            last_commit: "N/A".to_owned(),
            todos: Vec::new(),
        }
    }
    // TODO this is
    pub fn cycle_status(&mut self) {
        self.status = match self.status {
            ProjectStatus::Stable => ProjectStatus::Unstable,
            ProjectStatus::Unstable => ProjectStatus::Stable,
        };
        // TODO we need to write this
        update_project_status(self);
    }
}

#[derive(Default)]
pub struct ProjectBuilder {
    id: u8,
    sort: u8,
    path: String,
    name: String,
    desc: String,
    category: String,
    status: ProjectStatus,
    is_git: bool,
    owner: String,
    repo: String,
    last_commit: String,
    todos: Vec<Todo>,
}

impl ProjectBuilder {
    pub fn new() -> ProjectBuilder {
        ProjectBuilder::default()
    }
    pub fn id(mut self, id: u8) -> Self {
        self.id = id;
        self
    }
    pub fn sort(mut self, sort: u8) -> Self {
        self.sort = sort;
        self
    }
    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn desc(mut self, desc: String) -> Self {
        self.desc = "NA".to_string();
        self
    }
    pub fn category(mut self, category: String) -> Self {
        self.category = category;
        self
    }
    pub fn status(mut self, status: ProjectStatus) -> Self {
        self.status = status;
        self
    }
    pub fn is_git(mut self, is_git: bool) -> Self {
        self.is_git = is_git;
        self
    }
    pub fn owner(mut self, owner: String) -> Self {
        self.owner = owner;
        self
    }
    pub fn repo(mut self, repo: String) -> Self {
        self.repo = repo;
        self
    }
    pub fn last_commit(mut self, last_commit: String) -> Self {
        self.last_commit = last_commit;
        self
    }
    pub fn todos(mut self, todos: Vec<Todo>) -> Self {
        self.todos = todos;
        self
    }
    pub fn build(self) -> Project {
        Project {
            id: self.id, // TODO default
            sort: self.sort,
            path: self.path,
            name: self.name,
            desc: self.desc,
            category: self.category,
            status: self.status,
            is_git: self.is_git,
            owner: self.owner,
            repo: self.repo,
            last_commit: self.last_commit,
            todos: self.todos,
        }
    }
}

impl TableItems<Project> {
    pub fn load() -> TableItems<Project> {
        TableItems {
            items: Project::load(),
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
    pub fn current_todos(&self) -> Option<Vec<Todo>> {
        self.current().and_then(|p| Some(p.todos))
    }
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].cycle_status();
            } else {
                continue;
            }
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, EnumString, Serialize, Deserialize)]
pub enum ProjectStatus {
    #[default]
    Stable,
    Unstable,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TODO refactor to include-sql
// TODO remove FromSql
// impl FromSql for ProjectStatus {
//     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
//         value
//             .as_str()?
//             .parse::<ProjectStatus>()
//             .map_err(|e| FromSqlError::Other(Box::new(e)))
//     }
// }
