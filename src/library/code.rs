use crate::library::gitconfig::guess_git_owner;
use crate::library::sql::{
    initialize_db,
    read_project,
    read_tmp,
    read_todo,
    regex_last_dir,
    regex_repo, // TODO move regex repo to another folder
    update_project_category,
    update_project_desc,
    update_project_status,
    update_tmp,
    update_todo,
    write_new_todo,
    write_project,
    write_tmp_to_project,
};
use glob::glob;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
    Connection,
};
use shellexpand;
use std::path::PathBuf;
use std::{env, fmt};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use tui::style::Color;
use tui::widgets::{ListState, TableState};
use tui_textarea::{Input, Key, TextArea};

// use super::new_sql::update_project_category;

/// SQL
/// // TODO needs ot be dynamic
pub const DATABASE: &str = "projects.db";
pub const SEARCH_DIRECTORY_PREFIX: &str = "~/Documents/";
pub const CONFIG_PATH_SUFFIX: &str = "**/.git/config";
pub const SUBPATH_GIT_CONFIG: &str = ".git/config";
pub const CONFIG_SEARCH_PREFIX: &str = "~/Documents/";
pub const SUB_HOME_FOLDER: &str = "/Documents/";

/// UI
pub const HIGHLIGHT_SYMBOL: &str = ">> ";

pub fn fetch_config_files() -> Vec<String> {
    let expanded_path = shellexpand::tilde(CONFIG_SEARCH_PREFIX);
    let pattern: PathBuf = [&expanded_path, CONFIG_PATH_SUFFIX].iter().collect();

    let tmp: Vec<String> = glob(pattern.to_str().unwrap())
        .expect("expectation!!")
        .filter_map(|x| x.ok())
        .map(|x| {
            x.into_os_string()
                .into_string()
                .unwrap()
                .replace(SUBPATH_GIT_CONFIG, "")
        })
        .collect();

    tmp
}

/// APP
pub struct App {
    pub message: String,
    pub window: Window,
    pub configs: TableItems<GitConfig>,
    pub projects: TableItems<Project>,
    pub todos: FilteredListItems<Todo>,
    pub categories: ListItems<Category>,
}

impl App {
    fn load(conn: Option<Connection>) -> App {
        App {
            message: "hiii".to_owned(),
            window: Window::new(),
            configs: TableItems::<GitConfig>::load(),
            projects: TableItems::<Project>::load(conn),
            todos: FilteredListItems::<Todo>::load(None),
            categories: ListItems::<Category>::new(),
        }
    }
    pub fn init() -> App {
        let conn = Connection::open(DATABASE).unwrap();
        // INITIALIZE DB
        initialize_db();

        App::load(Some(conn))
    }
    pub fn next(&mut self) {
        match self.window {
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Project,
                ..
            } => {
                self.projects.next();
                // TODO move to update
                match self.projects.current() {
                    Some(p) => {
                        let items = self.todos.items.clone();
                        self.todos.filtered =
                            items.into_iter().filter(|t| t.project_id == p.id).collect();
                        self.todos.select_state(Some(0));
                    }
                    None => {}
                }
            }
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Todo,
                ..
            } => self.todos.next(),
            Window {
                popup: PopupWindow::SearchGitConfig,
                base: _,
                ..
            } => self.configs.next(),
            Window {
                popup: PopupWindow::EditCategory,
                base: _,
                ..
            } => self.categories.next(),
            _ => {}
        }
    }
    pub fn previous(&mut self) {
        match self.window {
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Project,
                ..
            } => {
                self.projects.previous();
                // TODO move to update
                match self.projects.current() {
                    Some(p) => {
                        let items = self.todos.items.clone();
                        self.todos.filtered =
                            items.into_iter().filter(|t| t.project_id == p.id).collect();
                        self.todos.select_state(Some(0));
                    }
                    None => {}
                }
            }
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Todo,
                ..
            } => self.todos.previous(),
            Window {
                popup: PopupWindow::SearchGitConfig,
                base: _,
                ..
            } => self.configs.previous(),
            Window {
                popup: PopupWindow::EditCategory,
                base: _,
                ..
            } => self.categories.previous(),
            _ => {}
        }
    }
    pub fn cycle_focus_next(&mut self) {
        self.window.base = match self.window.base.clone() {
            BaseWindow::Project => BaseWindow::Todo,
            BaseWindow::Todo => BaseWindow::Description,
            BaseWindow::Description => BaseWindow::Project,
        }
    }
    pub fn cycle_focus_previous(&mut self) {
        self.window.base = match self.window.base.clone() {
            BaseWindow::Project => BaseWindow::Description,
            BaseWindow::Todo => BaseWindow::Project,
            BaseWindow::Description => BaseWindow::Todo,
        }
    }
    pub fn default_select(&mut self) {
        // TODO what if there aren't any?
        self.projects.state.select(Some(0));
        self.configs.state.select(Some(0));
        self.categories.state.select(Some(0));

        match self.projects.current() {
            Some(p) => self.todos.select_filter_state(Some(0), p.id),
            None => (),
        }
    }
    pub fn add_project_in_dir(&mut self, is_find_git: bool) {
        match self.window {
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Project,
                ..
            } => {
                // TODO discover the higher git repo
                let path = env::current_dir().unwrap().display().to_string();
                if is_find_git {
                    let repo = match git2::Repository::discover(path) {
                        Ok(r) => r.workdir().unwrap().to_str().unwrap().to_string(),
                        _ => "N/A".to_string(),
                    };
                    write_project(Project {
                        id: 0,
                        path: repo.clone(),
                        name: regex_last_dir(repo.clone()),
                        desc: "N/A".to_owned(),
                        category: Category::Unknown,
                        status: ProjectStatus::Unstable,
                        is_git: true,
                        owner: guess_git_owner(repo.clone()), //TODO
                        repo: regex_last_dir(repo.clone()),
                        last_commit: "N/A".to_owned(),
                    });
                } else {
                    write_project(Project {
                        id: 0,
                        path: path.clone(),
                        name: regex_last_dir(path.clone()),
                        desc: "N/A".to_owned(),
                        category: Category::Unknown,
                        status: ProjectStatus::Unstable,
                        is_git: false,
                        owner: "quffaro".to_owned(), //TODO
                        repo: "".to_owned(),         //TODO should be null sql
                        last_commit: "N/A".to_owned(),
                    });
                }
                self.projects = TableItems::<Project>::load(None);
                self.projects.select_state(Some(0));
            }
            _ => {}
        }
    }
    /// WINDOW RULES
    pub fn popup(&mut self, popup: PopupWindow, mode: Option<Mode>) {
        self.window.popup = popup;
        match mode {
            Some(m) => self.window.mode = m,
            None => (),
        }
    }
    pub fn close_popup(&mut self) {
        self.window.popup = PopupWindow::None;
        self.window.status = WindowStatus::NotLoaded;
    }
    pub fn popup_mode_insert(&mut self, textarea: &mut TextArea) {
        match crossterm::event::read().expect("POPUP INSERT ERROR").into() {
            Input {
                key: Key::Char('c'),
                ctrl: true,
                ..
            }
            | Input { key: Key::Esc, .. } => self.window.mode = Mode::Normal,
            Input {
                key: Key::Enter, ..
            } => {}
            input => {
                textarea.input(input);
            }
        }
    }
    pub fn popup_mode_normal(&mut self, textarea: &mut TextArea, popup: PopupWindow) {
        match crossterm::event::read().expect("POPUP NORMAL ERROR").into() {
            Input {
                key: Key::Char('i'),
                ..
            } => self.window.mode = Mode::Insert,
            Input {
                key: Key::Char('q'),
                ..
            } => {
                self.close_popup();
                *textarea = TextArea::default();
            }
            Input {
                key: Key::Char('w'),
                ..
            } => {
                self.popup_write_and_close(textarea, popup);
                *textarea = TextArea::default();
            }
            Input { key: Key::Down, .. }
            | Input {
                key: Key::MouseScrollDown,
                ..
            } => self.next(),
            Input { key: Key::Up, .. }
            | Input {
                key: Key::MouseScrollUp,
                ..
            } => self.previous(),
            Input {
                key: Key::Enter, ..
            } => self.toggle(),
            _ => {}
        }
    }
    pub fn popup_write_and_close(&mut self, textarea: &mut TextArea, popup: PopupWindow) {
        let content = textarea.lines().join("\n").to_owned();
        match popup {
            PopupWindow::AddTodo => {
                let project_id = match self.projects.current() {
                    Some(p) => p.id,
                    None => 0,
                };
                write_new_todo(vec![Todo {
                    id: 0,
                    parent_id: 0,
                    project_id: project_id,
                    todo: content,
                    is_complete: false,
                }]);

                self.todos = FilteredListItems::<Todo>::load(None);
                self.todos.select_filter_state(Some(0), project_id);
            }
            PopupWindow::EditDesc => match self.projects.current_state() {
                (Some(idx), Some(p)) => {
                    update_project_desc(p, content).expect("A");
                    // reload projects but retain selection
                    self.projects = TableItems::<Project>::load(None);
                    self.projects.select_state(Some(idx));
                }
                _ => (),
            },
            PopupWindow::EditCategory => match self.projects.current_state() {
                (Some(idx), Some(p)) => match self.categories.current() {
                    Some(cat) => {
                        update_project_category(p, cat);
                        self.projects = TableItems::<Project>::load(None);
                        self.projects.select_state(Some(idx));
                    }
                    _ => {}
                },
                _ => (),
            },
            PopupWindow::SearchGitConfig => {
                write_tmp_to_project();
                self.projects = TableItems::<Project>::load(None);
                self.projects.select_state(Some(0));
            }
            _ => (),
        }
        self.close_popup();
    }
    pub fn toggle(&mut self) {
        match self.window {
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Project,
                ..
            } => self.projects.toggle(),
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Todo,
                ..
            } => self.todos.toggle(),
            Window {
                popup: PopupWindow::SearchGitConfig,
                base: _,
                ..
            } => self.configs.toggle(),
            Window {
                popup: PopupWindow::EditCategory,
                base: _,
                ..
            } => match self.projects.current() {
                Some(p) => self.categories.toggle(p),
                None => {}
            },
            _ => {}
        }
    }

    pub fn delete_todo(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u8,
    pub path: String,
    pub name: String,
    pub desc: String,
    pub category: Category,
    pub status: ProjectStatus,
    pub is_git: bool,
    pub owner: String,
    pub repo: String,
    pub last_commit: String,
}

impl Project {
    pub fn load(conn: Option<Connection>) -> Vec<Project> {
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
            category: Category::Unknown,
            status: ProjectStatus::Unstable,
            is_git: false,
            owner: "".to_owned(), //TODO
            repo: current_dir.clone(),
            last_commit: "N/A".to_owned(),
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

impl TableItems<Project> {
    pub fn load(conn: Option<Connection>) -> TableItems<Project> {
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

/// STRUCTS
pub trait ListNavigate {
    fn get_items_len<'a>(&'a self) -> usize;
    fn get_state_selected<'a>(&'a self) -> Option<usize>;
    fn select_state<'a>(&'a mut self, idx: Option<usize>);
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

// TODO

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
pub struct GitConfig {
    pub path: String,
    pub is_selected: bool,
}

impl GitConfig {
    pub fn load() -> Vec<GitConfig> {
        // TODO unwrap
        read_tmp(None).unwrap()
        // vec![]
    }
}

impl TableItems<GitConfig> {
    pub fn load() -> TableItems<GitConfig> {
        TableItems {
            items: GitConfig::load(),
            state: TableState::default(),
        }
    }
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].is_selected = !self.items[i].is_selected;
                update_tmp(&self.items[i]); // TODO
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
    pub fn load(conn: Option<Connection>) -> FilteredListItems<Todo> {
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

/// ENUMS
/// WINDOWS

pub struct Window {
    pub base: BaseWindow,
    pub popup: PopupWindow,
    pub status: WindowStatus,
    pub mode: Mode,
}

impl Window {
    fn new() -> Window {
        Window {
            base: BaseWindow::Project,
            popup: PopupWindow::None,
            status: WindowStatus::NotLoaded,
            mode: Mode::Insert,
        }
    }
    pub fn mode_color(&self) -> Color {
        match self.mode {
            Mode::Insert => Color::Yellow,
            Mode::Normal => Color::Green,
        }
    }
    pub fn base_focus_color(&self, window: BaseWindow) -> Color {
        match self {
            Window {
                popup: PopupWindow::None,
                base: window,
                ..
            } => Color::Yellow,
            _ => Color::White,
        }
    }
    fn to_project() -> Window {
        Window {
            base: BaseWindow::Project,
            popup: PopupWindow::None,
            status: WindowStatus::NotLoaded,
            mode: Mode::Insert,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter, EnumString)]
pub enum BaseWindow {
    Project,
    Todo,
    Description,
}

impl fmt::Display for BaseWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Project => write!(f, "PROJECTS"),
            Self::Todo => write!(f, "TODO"),
            Self::Description => write!(f, "DESC"),
        }
    }
}

impl ListItems<BaseWindow> {
    fn new() -> ListItems<BaseWindow> {
        ListItems {
            items: BaseWindow::iter().collect(),
            state: ListState::default(),
        }
    }
    pub fn current(&mut self) -> Option<&BaseWindow> {
        let idx = self.get_state_selected().unwrap();
        self.items.iter().nth(idx)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter, EnumString)]
pub enum PopupWindow {
    None,
    SearchGitConfig,
    AddTodo,
    EditCategory,
    EditDesc,
    Help,
}

impl ListItems<PopupWindow> {
    fn new() -> ListItems<PopupWindow> {
        ListItems {
            items: PopupWindow::iter().collect(),
            state: ListState::default(),
        }
    }
    fn current(&mut self) -> &PopupWindow {
        let idx = self.get_state_selected().unwrap();
        &self.items.iter().nth(idx).unwrap()
    }
}

pub enum WindowStatus {
    Loaded,
    NotLoaded,
}

#[derive(PartialEq, Eq, Debug, Clone, EnumString, EnumIter)]
pub enum Category {
    Unknown,
    Math,
    Haskell,
    OCaml,
    Rust,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromSql for Category {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse::<Category>()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ListItems<Category> {
    pub fn new() -> ListItems<Category> {
        ListItems {
            items: Category::iter().collect(),
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
                update_project_category(project, &self.items[i]);
            } else {
                continue;
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
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
