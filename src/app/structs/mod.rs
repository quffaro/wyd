use ratatui::widgets::{ListState, TableState};

pub mod category;
pub mod config;
pub mod gitconfig;
pub mod jobs;
pub mod projects;
pub mod todos;
pub mod windows;

pub trait ListNav {
    fn default() -> Self;
    fn get_items_len<'a>(&'a self) -> usize;
    fn get_state_selected<'a>(&'a self) -> Option<usize>;
    fn select_state<'a>(&'a mut self, idx: Option<usize>);
    // SUBSTATE
    fn next(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_state(Some(0))
                } else {
                    self.select_state(Some(i + 1))
                };
            }
            (None, _) => self.select_state(Some(0)),
        }
    }
    fn previous(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(0), l) => self.select_state(Some(l - 1)),
            (Some(i), _) => self.select_state(Some(i - 1)),
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

impl<T> ListNav for ListItems<T> {
    fn default() -> ListItems<T> {
        ListItems {
            items: vec![],
            state: ListState::default(),
        }
    }
    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }

    fn next(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_state(Some(0))
                } else {
                    self.select_state(Some(i + 1))
                };
            }
            (None, _) => self.select_state(Some(0)),
        }
    }
    fn previous(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(0), l) => self.select_state(Some(l - 1)),
            (Some(i), _) => self.select_state(Some(i - 1)),
            (None, _) => self.select_state(Some(0)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableItems<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> ListNav for TableItems<T> {
    fn default() -> TableItems<T> {
        TableItems {
            items: vec![],
            state: TableState::default(),
        }
    }

    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }

    fn next(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_state(Some(0))
                } else {
                    self.select_state(Some(i + 1))
                };
            }
            (None, _) => self.select_state(Some(0)),
        }
    }
    fn previous(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(0), l) => self.select_state(Some(l - 1)),
            (Some(i), _) => self.select_state(Some(i - 1)),
            (None, _) => self.select_state(Some(0)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NestedTableItems<T> {
    pub items: Vec<T>,
    pub state: TableState,
    pub substate_count: usize,
    pub substate: TableState,
}

impl<T> ListNav for NestedTableItems<T> {
    fn default() -> Self {
        NestedTableItems {
            items: vec![],
            state: TableState::default(),
            substate_count: 0,
            substate: TableState::default(),
        }
    }

    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }
    fn next(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_state(Some(0));
                } else {
                    self.select_state(Some(i + 1))
                };
            }
            (None, _) => self.select_state(Some(0)),
        }
    }
    fn previous(&mut self) {
        match (self.get_state_selected(), self.get_items_len()) {
            (_, 0) => {}
            (Some(0), l) => self.select_state(Some(l - 1)),
            (Some(i), _) => self.select_state(Some(i - 1)),
            (None, _) => self.select_state(Some(0)),
        }
    }
}
pub trait SubListNav {
    fn get_subitems_len<'a>(&'a self) -> usize;
    fn get_substate_selected<'a>(&'a self) -> Option<usize>;
    fn select_substate<'a>(&'a mut self, idx: Option<usize>);
    fn set_substate_count(&mut self, count: usize);
    fn next_substate(&mut self) {
        match (self.get_substate_selected(), self.get_subitems_len()) {
            (_, 0) => {}
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_substate(Some(0))
                } else {
                    self.select_substate(Some(i + 1))
                };
            }
            (None, _) => self.select_substate(Some(0)),
        }
    }
    fn previous_substate(&mut self) {
        match (self.get_substate_selected(), self.get_subitems_len()) {
            (_, 0) => {}
            (Some(0), l) => self.select_substate(Some(l - 1)),
            (Some(i), _) => self.select_substate(Some(i - 1)),
            (None, _) => self.select_substate(Some(0)),
        }
    }
}
impl<T> SubListNav for NestedTableItems<T> {
    fn set_substate_count(&mut self, count: usize) {
        self.substate_count = count;
    }
    fn get_substate_selected<'a>(&'a self) -> Option<usize> {
        self.substate.selected()
    }
    fn select_substate<'a>(&'a mut self, idx: Option<usize>) {
        self.substate.select(idx)
    }

    fn get_subitems_len<'a>(&'a self) -> usize {
        self.substate_count
    }
    fn next_substate(&mut self) {
        match (self.get_substate_selected(), self.get_subitems_len()) {
            (_, 0) => {}
            (Some(i), l) => {
                if i >= l - 1 {
                    self.select_substate(Some(0))
                } else {
                    self.select_substate(Some(i + 1))
                };
            }
            (None, _) => self.select_substate(Some(0)),
        }
    }
    fn previous_substate(&mut self) {
        match (self.get_substate_selected(), self.get_subitems_len()) {
            (_, 0) => {}
            (Some(0), l) => self.select_substate(Some(l - 1)),
            (Some(i), _) => self.select_substate(Some(i - 1)),
            (None, _) => self.select_substate(Some(0)),
        }
    }
}
