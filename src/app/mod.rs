use self::regex::regex_last_dir;
use crate::app::structs::{
    gitconfig::guess_git_owner,
    projects::{Project, ProjectStatus},
    todos::Todo,
    windows::{Mode, Popup, Window},
    FilteredListItems, ListNav, TableItems,
};
use crate::app::ui::{render_popup_todo, render_projects, render_title, render_todo_and_desc};
use crate::sql::project::write_project;
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX, PATH_DB};
use dirs::home_dir;
use git2::Repository;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::terminal::Frame;
use rusqlite::Connection;
use std::{env, error};
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
    pub input: Input,
    pub window: Window,
    pub projects: TableItems<Project>,
    pub todos: FilteredListItems<Todo>,
    pub desc: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: Input::default(),
            window: Window::new(false),
            projects: TableItems::<Project>::default(),
            todos: FilteredListItems::<Todo>::default(),
            desc: "".to_owned(),
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
        Self {
            running: true,
            input: Input::default(),
            window: Window::new(false),
            projects: TableItems::<Project>::load(&conn),
            todos: FilteredListItems::<Todo>::load(&conn),
            desc: "".to_owned(),
        }
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/latest/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/master/examples
        let size = frame.size();

        // Vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(size);

        // CHUNK 0
        let title = render_title(&self);
        frame.render_widget(title, chunks[0]);

        // CHUNK 1
        let projects = render_projects(&self);
        frame.render_stateful_widget(projects, chunks[1], &mut self.projects.state);

        // CHUNK 2
        let project_info_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);

        let (todo_block, desc_block) = render_todo_and_desc(self);
        frame.render_stateful_widget(todo_block, project_info_chunk[0], &mut self.todos.state);
        frame.render_widget(desc_block, project_info_chunk[1]);

        // POPUP
        match self.window.popup {
            Popup::AddTodo => render_popup_todo(self, frame),
            _ => {}
        }
    }
    // TODO move body into another function and have this one reload it
    pub fn add_project_in_dir(&mut self, is_find_git: bool) {
        let path = env::current_dir().unwrap().display().to_string();
        if is_find_git {
            let repo = match git2::Repository::discover(path) {
                Ok(r) => r.workdir().unwrap().to_str().unwrap().to_string(),
                _ => "N/A".to_string(),
            };
            write_project(
                &Connection::open(home_path(PATH_DB)).unwrap(),
                Project {
                    id: 0,
                    path: repo.clone(),
                    name: regex_last_dir(repo.clone()),
                    desc: "N/A".to_owned(),
                    category: "Unknown".to_owned(),
                    status: ProjectStatus::Unstable,
                    is_git: true,
                    owner: guess_git_owner(repo.clone()), //TODO
                    repo: regex_last_dir(repo.clone()),
                    last_commit: "N/A".to_owned(),
                },
            );
        } else {
            write_project(
                &Connection::open(home_path(PATH_DB)).unwrap(),
                Project {
                    id: 0,
                    path: path.clone(),
                    name: regex_last_dir(path.clone()),
                    desc: "N/A".to_owned(),
                    category: "Unknown".to_owned(),
                    status: ProjectStatus::Unstable,
                    is_git: false,
                    owner: "quffaro".to_owned(), //TODO
                    repo: "".to_owned(),         //TODO should be null sql
                    last_commit: "N/A".to_owned(),
                },
            );
        }
    }
    pub fn popup(&mut self, popup: Popup, mode: Option<Mode>) {
        self.window.popup(popup, mode)
    }
    pub fn close_popup(&mut self) {
        self.window.close_popup()
    }
}
