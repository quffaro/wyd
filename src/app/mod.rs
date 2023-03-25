use self::actions::Actions;
use self::state::AppState;
use self::structs::{projects::Project, todos::Todo, FilteredListItems, TableItems};
use super::sql::initialize_db;
use super::{home_path, PATH_DB};
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;
use rusqlite::Connection;

pub mod actions;
pub mod regex;
pub mod state;
pub mod structs;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    is_loading: bool,
    state: AppState,
    // data states go here
    projects: TableItems<Project>,
    todos: FilteredListItems<Todo>,
}

impl App {
    // TODO rename to load
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let conn = &Connection::open(home_path(PATH_DB)).unwrap();
        initialize_db(conn);
        let projects = TableItems::<Project>::load(conn);
        let todos = FilteredListItems::<Todo>::load(conn);

        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
            projects,
            todos,
        }
    }
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
            }
        } else {
            AppReturn::Continue
        }
    }

    pub async fn do_nothing(&mut self) -> AppReturn {
        AppReturn::Continue
    }

    pub async fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self) {
        self.actions = vec![Action::Quit].into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}
