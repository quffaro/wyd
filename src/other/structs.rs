use super::sql::{
    read_project, read_tmp, read_todo, update_project_status, update_tmp, update_todo,
};
use strum::IntoEnumIterator;
use tui::widgets::{ListState, TableState};
use wyd::{Category, Mode, Status, WindowStatus};

pub struct Window {
    pub focus: String,
    pub status: WindowStatus,
    pub mode: Mode,
}

pub trait ListNavigate {
    fn get_items_len<'a>(&'a self) -> usize;
    fn get_state_selected<'a>(&'a self) -> Option<usize>;
    fn select_state<'a>(&'a mut self, idx: Option<usize>);
    //
    fn next(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {},
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_state(Some(0))
                } else {
                    self.select_state(Some(i + 1))
                };
            },
            (None, _) => self.select_state(Some(0)),
        }

        // let i = match self.get_state_selected() {
        //     Some(i) => {
        //         if i >= self.get_items_len() - 1 {
        //             0
        //         } else {
        //             i + 1
        //         }
        //     }
        //     None => 0,
        // };
        // self.select_state(Some(i));
    }
    fn previous(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {},
            (Some(0), l) => self.select_state(Some(l - 1)),
            (Some(i), _) => self.select_state(Some(i + 1)),
            (None, _) => self.select_state(Some(0)),
        }
    }
    fn unselect(&mut self) {
        self.select_state(None);
    }
}

#[derive(Clone, Debug)]
pub struct ListItems<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> ListNavigate for ListItems<T> {
    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }
}

impl ListItems<Category> {
    pub fn new() -> ListItems<Category> {
        ListItems {
            items: Category::iter().collect(),
            state: ListState::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FilteredListItems<T> {
    pub items: Vec<T>,
    pub filtered: Vec<T>,
    pub state: ListState,
}

impl<T> ListNavigate for FilteredListItems<T> {
    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }
}

#[derive(Clone, Debug)]
pub struct TableItems<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> ListNavigate for TableItems<T> {
    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }
}

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub path: String,
    pub is_selected: bool,
}

impl TableItems<GitConfig> {
    pub fn new() -> TableItems<GitConfig> {
        TableItems {
            // items: Vec::<GitConfig>::new(),
            items: read_tmp().unwrap(),
            state: TableState::default(),
        }
    }
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].is_selected = !self.items[i].is_selected;
                update_tmp(&self.items[i]);
            } else {
                continue;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u8,
    pub path: String,
    pub name: String,
    pub desc: String,
    pub category: Category,
    pub status: Status,
    pub is_git: bool,
    pub last_commit: String,
}

impl Project {
    pub fn cycle_status(&mut self) {
        self.status = match self.status {
            Status::Stable => Status::Unstable,
            Status::Unstable => Status::Stable,
        };
        // TODO we need to write this
        update_project_status(self);
    }
}

impl TableItems<Project> {
    pub fn new() -> TableItems<Project> {
        TableItems {
            items: read_project().expect("AA"),
            state: TableState::default(),
        }
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

// TODO i would like to nest these guys
#[derive(Clone, Debug)]
pub struct Todo {
    pub id: u8,
    pub parent_id: u8,
    pub project_id: u8,
    pub todo: String,
    pub is_complete: bool,
}

impl FilteredListItems<Todo> {
    pub fn new() -> FilteredListItems<Todo> {
        FilteredListItems {
            items: read_todo().expect("AA"),
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
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.filtered.len() {
            if i == selected {
                self.filtered[i].is_complete = !self.filtered[i].is_complete;
                update_todo(&self.filtered[i]).expect("msg");
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
    }
}
