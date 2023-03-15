use crate::refactor::new_sql::write_project;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use std::{env, fmt};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use tui::widgets::{ListState, TableState};
use tui::{
    style::Color,
    widgets::{List, Table},
};
use tui_textarea::{Input, Key, TextArea};

/// SQL
/// // TODO needs ot be dynamic
pub const DATABASE: &str = "projects.db";
pub const SEARCH_DIRECTORY_PREFIX: &str = "~/Documents/"; // CUFFARO IS NOT GUARANTEED!
pub const CONFIG_PATH_SUFFIX: &str = "**/.git/config";
pub const CONFIG_SEARCH_PREFIX: &str = "~/Documents/";

/// UI
pub const HIGHLIGHT_SYMBOL: &str = ">> ";

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

// TODO
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
            (Some(i), _) => self.select_state(Some(i + 1)),
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
        // read_tmp().unwrap()
        vec![]
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
                // update_tmp(&self.items[i]); // TODO
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
    pub status: ProjectStatus,
    pub is_git: bool,
    pub last_commit: String,
}

impl Project {
    pub fn load() -> Vec<Project> {
        // read_project().expect("READ PROJECT ERROR")
        vec![]
    }
    pub fn new_in_pwd() -> Project {
        let current_dir = env::current_dir().unwrap().display().to_string();
        let name = current_dir.clone();
        Project {
            id: 0,
            path: current_dir,
            name: name,
            desc: "".to_owned(),
            category: Category::Unknown,
            status: ProjectStatus::Unstable,
            is_git: false,
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
        // update_project_status(self);
    }
}

impl TableItems<Project> {
    pub fn load() -> TableItems<Project> {
        TableItems {
            items: Project::load(),
            state: TableState::default(),
        }
    }
    pub fn current(&mut self) -> Option<&Project> {
        let idx = self.get_state_selected().unwrap();
        self.items.iter().nth(idx)
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
    pub fn load() -> FilteredListItems<Todo> {
        FilteredListItems {
            items: vec![],
            // read_todo().expect("AA"),
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
                // update_todo(&self.filtered[i]).expect("msg");
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

/// WINDOWS
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

#[derive(EnumIter, EnumString)]
pub enum PopupWindow {
    None,
    SearchGitConfig,
    AddTodo,
    EditCategory,
    EditDesc,
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

/// ENUMS
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
    fn new() -> App {
        App {
            message: "hiii".to_owned(),
            window: Window::new(),
            configs: TableItems::<GitConfig>::load(),
            projects: TableItems::<Project>::load(),
            todos: FilteredListItems::<Todo>::load(),
            categories: ListItems::<Category>::new(),
        }
    }
    pub fn init() -> App {
        // INITIALIZE DB
        App::new()
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
    pub fn add_project_in_dir(&mut self) {
        // write_project(Project::new_in_pwd()); // SQL
        match self.window {
            Window {
                popup: PopupWindow::None,
                base: BaseWindow::Project,
                ..
            } => {
                write_project(Project {
                    id: 0,
                    path: env::current_dir().unwrap().display().to_string(),
                    name: "".to_owned(),
                    desc: "".to_owned(),
                    category: Category::Unknown,
                    status: ProjectStatus::Unstable,
                    is_git: false,
                    last_commit: "N/A".to_owned(),
                });
                self.projects = TableItems::<Project>::load();
            }
            _ => {}
        }
    }
    /// WINDOW RULES
    pub fn popup(&mut self, popup: PopupWindow) {
        // self.window.popup = popup
    }
    pub fn close_popup(&mut self) {
        self.window.popup = PopupWindow::None;
    }
    /// INPUT
    pub fn input(&mut self, textarea: &mut TextArea) {
        match self.window {
            Window {
                popup: PopupWindow::None,
                base: _,
                ..
            } => {
                if let Event::Key(key) = event::read().expect("Key Error") {
                    match key.code {
                        KeyCode::Char('d') => self.delete_todo(),
                        KeyCode::Char('i') => self.popup(PopupWindow::AddTodo),
                        KeyCode::Char(';') | KeyCode::Right => self.cycle_focus_next(),
                        KeyCode::Char('j') | KeyCode::Left => self.cycle_focus_previous(),
                        KeyCode::Char('l') | KeyCode::Down => self.next(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous(),
                        KeyCode::Enter => self.toggle(),
                        _ => {}
                    }
                }
            }
            Window {
                popup: _, base: _, ..
            } => match self.window.mode {
                Mode::Insert => self.popup_mode_insert(textarea),
                Mode::Normal => self.popup_mode_normal(textarea),
            },
            _ => {}
        }
    }
    // TODO result
    pub fn popup_mode_insert(&mut self, textarea: &mut TextArea) {
        match crossterm::event::read().expect("POPUP INSERT").into() {
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
    pub fn popup_mode_normal(&mut self, textarea: &mut TextArea) {
        match crossterm::event::read().expect("POPUP ERROR").into() {
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
                // TODO parameterize write and close
                self.popup_write_and_close(textarea);
                *textarea = TextArea::default();
            }
            Input { key: Key::Down, .. } => self.next(),
            Input { key: Key::Up, .. } => self.previous(),
            Input {
                key: Key::Enter, ..
            } => self.toggle(),
            _ => {}
        }
    }
    pub fn next(&mut self) {}
    pub fn previous(&mut self) {}
    pub fn toggle(&mut self) {}
    pub fn popup_write_and_close(&mut self, textarea: &mut TextArea) {
        let content = textarea.lines().join("\n").to_owned();
    }
    pub fn delete_todo(&mut self) {}
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
}
