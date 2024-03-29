use self::structs::{
    category::Category,
    config::{load_config, wyd_to_color, Config},
    gitconfig::GitConfig,
    jobs::{JobRoster, LoadingState},
    projects::Project,
    todos::Todo,
    windows::{BaseWindow, Mode, Popup, Window},
    FilteredListItems, ListNav, PlainListItems, TableItems,
};
use crate::app::ui::{
    base::{
        render_projects, render_title, render_title_and_search, render_todo, render_todo_and_desc,
    },
    popup::render_popup_todo,
    render_loading,
};
use crate::sql::project::{update_project_desc, update_project_sort, write_project};
use crate::{home_path, PATH_DB};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::terminal::Frame;
use rusqlite::Connection;
use std::error;
use tui_input::Input;

pub mod regex;
pub mod structs;
pub mod ui;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub tick: u8,
    pub index: usize,
    pub msg: String,
    pub input: Input,
    pub is_popup_loading: bool,
    pub config: Option<Config>,
    pub window: Window,
    pub jobs: JobRoster,
    pub projects: TableItems<Project>,
    pub todos: FilteredListItems<Todo>,
    pub configs: TableItems<GitConfig>,
    pub categories: PlainListItems<Category>,
}

impl Default for App {
    fn default() -> Self {
        match load_config() {
            Some(c) => App {
                running: true,
                tick: 0,
                index: 0,
                msg: "Nothing".to_owned(),
                input: Input::default(),
                is_popup_loading: false,
                config: Some(c),
                window: Window::load(true),
                jobs: JobRoster::new(),
                projects: TableItems::<Project>::default(),
                todos: FilteredListItems::<Todo>::default(),
                configs: TableItems::<GitConfig>::default(),
                categories: PlainListItems::<Category>::default(),
                // desc: "".to_owned(),
            },
            None => App {
                running: true,
                tick: 0,
                index: 0,
                msg: "Nothing".to_owned(),
                input: Input::default(),
                is_popup_loading: false,
                config: None,
                window: Window::load(false),
                jobs: JobRoster::new(),
                projects: TableItems::<Project>::default(),
                todos: FilteredListItems::<Todo>::default(),
                configs: TableItems::<GitConfig>::default(),
                categories: PlainListItems::<Category>::default(),
                // desc: "".to_owned(),
            },
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads data into the app
    pub fn load() -> Self {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        match load_config() {
            Some(c) => Self {
                running: true,
                tick: 0,
                index: 0,
                msg: "Nothing".to_owned(),
                input: Input::default(),
                is_popup_loading: false,
                config: Some(c),
                window: Window::load(true),
                jobs: JobRoster::new(),
                projects: TableItems::<Project>::load(&conn),
                todos: FilteredListItems::<Todo>::load(&conn),
                configs: TableItems::<GitConfig>::load(&conn),
                categories: PlainListItems::<Category>::load(&conn),
            },
            None => Self {
                running: true,
                tick: 0,
                index: 0,
                msg: "Nothing".to_owned(),
                input: Input::default(),
                is_popup_loading: false,
                config: None,
                window: Window::load(false),
                jobs: JobRoster::new(),
                projects: TableItems::<Project>::load(&conn),
                todos: FilteredListItems::<Todo>::load(&conn),
                configs: TableItems::<GitConfig>::load(&conn),
                categories: PlainListItems::<Category>::load(&conn),
            },
        }
    }

    pub fn reload(&mut self) {
        // TODO should retain selection...
        let conn = Connection::open(home_path(PATH_DB)).unwrap();

        match self.projects.get_state_selected() {
            Some(idx) => {
                self.projects = TableItems::<Project>::load(&conn);
                self.projects.select_state(Some(idx));
            }
            _ => (),
        }
    }

    pub fn finish_github_request(&mut self) {
        self.jobs.gitcommit = crate::app::LoadingState::Finished;
        self.reload();
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.tick = (self.tick + 1) % 5;
    }

    pub fn next(&mut self) {
        match self.window {
            Window {
                popup: Popup::None,
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
                popup: Popup::None,
                base: BaseWindow::Todo,
                ..
            } => self.todos.next(),
            Window {
                popup: Popup::SearchGitConfigs,
                base: _,
                ..
            } => self.configs.next(),
            Window {
                popup: Popup::EditCat,
                base: _,
                ..
            } => self.categories.next(),
            _ => {}
        }
    }
    pub fn previous(&mut self) {
        match self.window {
            Window {
                popup: Popup::None,
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
                popup: Popup::None,
                base: BaseWindow::Todo,
                ..
            } => self.todos.previous(),
            Window {
                popup: Popup::SearchGitConfigs,
                base: _,
                ..
            } => self.configs.previous(),
            Window {
                popup: Popup::EditCat,
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
            BaseWindow::Search => BaseWindow::Project,
        }
    }
    pub fn cycle_focus_previous(&mut self) {
        self.window.base = match self.window.base.clone() {
            BaseWindow::Project => BaseWindow::Description,
            BaseWindow::Todo => BaseWindow::Project,
            BaseWindow::Description => BaseWindow::Todo,
            BaseWindow::Search => BaseWindow::Project,
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

    // toggle
    pub fn toggle(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        match self.window {
            Window {
                popup: Popup::None,
                base: BaseWindow::Project,
                ..
            } => self.projects.toggle(&conn),
            Window {
                popup: Popup::None,
                base: BaseWindow::Todo,
                ..
            } => self.todos.toggle(&conn),
            Window {
                popup: Popup::SearchGitConfigs,
                base: _,
                ..
            } => self.configs.toggle(&conn),
            Window {
                popup: Popup::EditCat,
                base: _,
                ..
            } => match self.projects.current() {
                Some(p) => self.categories.toggle(&conn, p),
                None => {}
            },
            _ => {}
        }
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let size = frame.size();

        // Vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(3),
                    Constraint::Percentage(50),
                    Constraint::Percentage(40),
                    Constraint::Min(3),
                ]
                .as_ref(),
            )
            .split(size);

        // CHUNK 0
        let top_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(chunks[0]);

        let (search, title) = render_title_and_search(&self);
        // let title = render_title(&self);
        frame.render_widget(title, top_chunk[0]);
        frame.render_widget(search, top_chunk[1]);

        // CHUNK 1
        let projects = render_projects(&self);
        frame.render_stateful_widget(projects, chunks[1], &mut self.projects.state);

        // CHUNK 2
        let project_info_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);

        let (todo_block, desc_block) = render_todo_and_desc(self);
        let todo_table = render_todo(self);
        frame.render_stateful_widget(todo_table, project_info_chunk[0], &mut self.todos.state);
        frame.render_widget(desc_block, project_info_chunk[1]);

        // CHUNK 3
        let loading = render_loading(self);
        frame.render_widget(loading, chunks[3]);

        // POPUP
        match self.window.popup {
            Popup::AddTodo => crate::app::ui::popup::render_popup_todo(self, frame),
            Popup::ReadTodo => crate::app::ui::popup::render_popup_read_todo(self, frame),
            Popup::EditDesc => crate::app::ui::popup::render_popup_edit_desc(self, frame),
            Popup::SearchGitConfigs => {
                crate::app::ui::popup::render_popup_search_config(self, frame)
            }
            Popup::Help => crate::app::ui::popup::render_popup_help_table(self, frame),
            Popup::NewCat => crate::app::ui::popup::render_popup_new_cat(self, frame),
            Popup::EditCat => crate::app::ui::popup::render_popup_new_cat(self, frame),
            Popup::Config => crate::app::ui::popup::render_popup_wyd_config(self, frame),
            Popup::DeleteProject => crate::app::ui::popup::render_popup_delete_project(self, frame),
            Popup::NewProject => crate::app::ui::popup::render_popup_new_project(self, frame),
            _ => {}
        }
    }

    /// TODO move body into another function and have this one reload it
    pub fn add_project_in_dir(&mut self, is_find_git: bool) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        // TODO i want a rule which handles whether this operation will be performed before calling
        // a SQL rule. SQL rules should only be called when a write is certain
        crate::sql::project::add_project_in_dir(is_find_git, &conn);
        self.reload();
        // self.projects = TableItems::<Project>::load(&conn);
    }

    pub fn to_search(&mut self) {
        self.window.to_search()
    }
    // POPUP RULES
    pub fn popup(&mut self, popup: Popup, mode: Option<Mode>) {
        self.is_popup_loading = true;
        self.window.popup(popup, mode)
    }
    pub fn close_popup(&mut self) {
        self.is_popup_loading = false;
        self.window.close_popup()
    }
    // INPUT RULES
    pub fn default_input(&mut self) {
        self.input = Input::default()
    }
    pub fn default_close(&mut self) {
        self.default_input();
        self.close_popup();
    }
    // SQL RULES
    fn write_project(&mut self, conn: &Connection, project: Project) {
        crate::sql::project::write_project(
            conn, // TODO constructor
            project,
        );

        self.projects = TableItems::<Project>::load(&conn);
    }

    pub fn write_close_new_project(&mut self, project: Project) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        self.write_project(&conn, project);
        self.default_close();
    }

    fn write_todo(&mut self, conn: &Connection) {
        let project_id = match self.projects.current() {
            Some(p) => p.id,
            None => 0,
        };
        crate::sql::todo::write_new_todo(
            conn,
            // TODO constructor
            vec![Todo {
                id: 0,
                parent_id: 0,
                project_id: project_id,
                todo: self.input.value().to_owned(),
                is_complete: false,
                priority: 1,
            }],
        );

        self.todos = FilteredListItems::<Todo>::load(conn);
        self.todos.select_filter_state(Some(0), project_id);
    }

    fn update_project_desc(&mut self, conn: &Connection) {
        match self.projects.current_state() {
            (i, Some(p)) => {
                crate::sql::project::update_project_desc(conn, p.id, self.input.value().to_owned());
                self.reload();
                self.projects.select_state(i);
            }
            _ => {}
        };
    }

    pub fn write_close_new_todo(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        self.write_todo(&conn);
        self.default_close();
    }

    pub fn write_close_description(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        self.update_project_desc(&conn);
        self.default_close();
    }

    pub fn write_close_gitconfig(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        crate::sql::tmp_config::write_tmp_to_project(&conn);
        self.projects = TableItems::<Project>::load(&conn);
        self.default_close();
    }

    pub fn write_close_new_category(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        match self.projects.current_state() {
            (i, Some(p)) => {
                let value = self.input.value().to_owned();
                crate::sql::category::write_category(&conn, &value);
                // TODO needs to work if above write is successful
                crate::sql::project::update_project_category(&conn, p, &value);
                // reload
                self.categories = PlainListItems::<Category>::load(&conn);
                self.projects = TableItems::<Project>::load(&conn);
                self.projects.select_state(i);
            }
            _ => {}
        }
        self.default_close();
    }

    pub fn write_close_edit_category(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        match self.projects.current_state() {
            (i, Some(p)) => {
                let category = self.categories.current().unwrap().name.to_string();
                crate::sql::project::update_project_category(&conn, p, &category);
                self.categories = PlainListItems::<Category>::load(&conn);
                self.projects = TableItems::<Project>::load(&conn);
                self.projects.select_state(i);
            }
            _ => {}
        }
        self.default_close();
    }

    pub fn cycle_priority(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        match self.todos.current_state() {
            (idx, Some(t)) => {
                let priority = (t.priority + 1) % 3;
                crate::sql::todo::update_todo_priority(&conn, t.id, priority);
                // TODO
                // self.todos = FilteredListItems::<Todo>::load(&conn);
                // self.todos.filter_from_projects(t.project_id.clone());
                // self.todos.select_state(idx);
                // self.todos.select_state(idx);
            }
            _ => (),
        }
    }

    pub fn delete_project(&mut self) {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        match self.projects.current() {
            Some(p) => {
                crate::sql::project::delete_project(&conn, p);
                self.projects = TableItems::<Project>::load(&conn);
                self.todos = FilteredListItems::<Todo>::load(&conn);
                self.default_close();
            }
            _ => (),
        }
    }
    pub fn get_bg_color(&self) -> ratatui::style::Color {
        self.config
            .clone()
            .and_then(|c| Some(wyd_to_color(c.color.bd)))
            .unwrap()
    }

    pub fn yank(&mut self) {
        if self.window.base == BaseWindow::Project {
            match self.projects.current() {
                Some(p) => {
                    self.index = p.id as usize;
                    self.window.to_insert()
                }
                None => {}
            }
        }
    }

    pub fn paste(&mut self) {
        if self.window.base == BaseWindow::Project {
            match self.projects.current() {
                Some(p) => {
                    let conn = Connection::open(home_path(PATH_DB)).unwrap();
                    update_project_sort(&conn, self.index, (p.sort + 1).into());
                    self.reload();
                    self.index = 0;
                    self.window.to_normal();
                }
                None => {}
            }
        }
    }
}
