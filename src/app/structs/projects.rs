// use crate::app::structs;
use super::{ListNav, ListState, NestedTableItems, SubListNav, TableItems, TableState};
use crate::app::structs::todos::Todo;
use crate::json::read_json;
use derive_builder::Builder;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{env, fmt};
use strum_macros::EnumString;

#[derive(Debug, Clone, Builder, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Project {
    pub id: u8, // TODO default 0
    pub parent_id: Option<u8>,
    pub child_ids: Vec<u8>,
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
    // TODO to Option
    pub fn load() -> Vec<Project> {
        let projects = read_json().unwrap();
        // vec![ProjectBuilder::new().build()];
        // read_project().expect("READ PROJECT ERROR");
        projects
        // projects.into_iter().sorted_by_key(|x| x.sort()).collect()
    }
    // TODO
    pub fn new_in_pwd() -> Project {
        let current_dir = env::current_dir().unwrap().display().to_string();
        let name = current_dir.clone();
        // TODO use builder
        Project {
            id: 0,
            parent_id: None,
            child_ids: vec![],
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
        // update_project_status(self);
    }

    pub fn get_todos_len(&self) -> usize {
        self.todos.len()
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo)
    }
}

// TODO I don't think we're doing nested list impl anymore but lets get it working first
impl NestedTableItems<Project> {
    pub fn load() -> NestedTableItems<Project> {
        let projects = Project::load();
        let substate_count = projects.iter().nth(0).map(|p| p.todos.len()).unwrap_or(0);
        NestedTableItems {
            items: projects,
            state: TableState::default(),
            substate_count: substate_count,
            substate: TableState::default(),
        }
    }

    pub fn update_substate_count(&mut self) {
        let substate_count = self.current().map(|p| p.get_todos_len()).unwrap_or(0);
        self.set_substate_count(substate_count);

        if substate_count == 0 {
            self.substate = TableState::default()
        } else {
            self.substate.select(Some(0))
        };
    }
    pub fn project_next(&mut self) {
        self.next();
        self.update_substate_count();
    }
    pub fn project_previous(&mut self) {
        self.previous();
        self.update_substate_count();
    }
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
    pub fn current_todo(&self) -> Option<&Todo> {
        match (self.get_state_selected(), self.get_substate_selected()) {
            (Some(idx), Some(idy)) => self
                .current()
                .iter()
                .nth(idx)
                .and_then(|p| p.todos.iter().nth(idy)),
            _ => None,
        }
    }
    // pub fn current_todo_state(&self) -> (Option<usize>, Option<&Todo>) {

    // }
    pub fn current_todos(&self) -> Option<&Vec<Todo>> {
        self.current().and_then(|p| Some(&p.todos))
    }

    // pub fn current_full_state(&self) -> (Option<usize>, Option<usize>, Option<&Project>) {

    // }

    //
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
