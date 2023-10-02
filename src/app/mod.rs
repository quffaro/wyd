use self::structs::{
    category::Category,
    config::{load_config, wyd_to_color, Config},
    gitconfig::GitConfig,
    jobs::{JobRoster, LoadingState},
    projects::Project,
    todos::{Todo, TodoBuilder},
    windows::{BaseWindow, Mode, Popup, Window},
    ListItems, ListNav, NestedTableItems, SubListNav, TableItems,
};
use crate::app::structs::projects::ProjectBuilder;
use crate::app::ui::{
    base::{
        render_projects, render_title, render_title_and_search, render_todo, render_todo_and_desc,
    },
    popup::render_popup_todo,
    render_loading,
};
use crate::json::{read_json, write_json};
use crate::{home_path, PATH_DB};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::terminal::Frame;
use std::error;
use std::fs;
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
    pub categories: ListItems<Category>,
    pub projects: NestedTableItems<Project>,
    pub configs: TableItems<GitConfig>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            tick: 0,
            index: 0,
            msg: "Nothing".to_owned(),
            input: Input::default(),
            is_popup_loading: false,
            config: load_config(),
            window: Window::load(true),
            jobs: JobRoster::new(),
            categories: ListItems::<Category>::default(),
            projects: NestedTableItems::<Project>::default(),
            configs: TableItems::<GitConfig>::default(),
        }
    }
}

impl App {
    pub fn test(&mut self) {
        self.projects.substate.select(Some(1))
    }

    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads data into the app
    pub fn load() -> Self {
        Self {
            running: true,
            tick: 0,
            index: 0,
            msg: "Nothing".to_owned(),
            input: Input::default(),
            is_popup_loading: false,
            config: load_config(),
            window: Window::load(true),
            jobs: JobRoster::new(),
            categories: ListItems::<Category>::load(),
            projects: NestedTableItems::<Project>::load(),
            configs: TableItems::<GitConfig>::load(),
        }
    }

    pub fn reload(&mut self) {
        // TODO ...
        let (x, y) = (
            self.projects.get_state_selected(),
            self.projects.get_substate_selected(),
        );

        match self.projects.get_state_selected() {
            x => {
                self.projects = NestedTableItems::<Project>::load();
                self.projects.select_state(x);
                self.projects.select_substate(y);
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
            } => self.projects.project_next(),
            Window {
                popup: Popup::None,
                base: BaseWindow::Todo,
                ..
            } => self.projects.next_substate(),
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
            } => self.projects.project_previous(),
            Window {
                popup: Popup::None,
                base: BaseWindow::Todo,
                ..
            } => self.projects.previous_substate(),
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
        self.projects.substate.select(Some(0));
        self.configs.state.select(Some(0));
        self.categories.state.select(Some(0));

        // match self.projects.current() {
        //     Some(p) => self.todos.select_filter_state(Some(0), p.id),
        //     None => (),
        // }
    }

    pub fn toggle(&mut self) {
        match self.window {
            Window {
                popup: Popup::None,
                base: BaseWindow::Project,
                ..
            } => self.projects.toggle(),
            Window {
                popup: Popup::None,
                base: BaseWindow::Todo,
                ..
            } => self.projects.toggle(),
            Window {
                popup: Popup::SearchGitConfigs,
                base: _,
                ..
            } => self.configs.toggle(),
            Window {
                popup: Popup::EditCat,
                base: _,
                ..
            } => match self.projects.current() {
                Some(p) => self.categories.toggle(p),
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
        frame.render_stateful_widget(
            todo_table,
            project_info_chunk[0],
            &mut self.projects.substate,
        );
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
    pub fn add_project_to_app(&mut self, project: Project) {
        self.projects.items.push(project);
        crate::json::project::write_projects(&self.projects.items);
        self.reload();
    }
    // TODO move body into another function and have this one reload it
    pub fn add_project_in_dir(&mut self, is_find_git: bool) {
        // TODO i want a rule which handles whether this operation will be performed before calling
        // let mut projects = &mut self.projects.items;
        // projects.push(ProjectBuilder::new().id(1).build());
        // crate::json::project::write_projects(projects);
        // self.reload();
        // TODO unwrap not ideal

        self.add_project_to_app(
            ProjectBuilder::default()
                .id(1)
                .parent_id(0)
                .child_ids(vec![])
                .sort(0)
                .path("".to_string())
                .name("".to_string())
                .desc("".to_string())
                .category("".to_string())
                .status(crate::app::structs::projects::ProjectStatus::Stable)
                .is_git(false)
                .owner("".to_string())
                .repo("".to_string())
                .last_commit("".to_string())
                .todos(vec![])
                .build()
                .unwrap(),
        );
    }
    pub fn current_project_name(&self) -> Option<&String> {
        self.projects.current().map(|p| &p.name)
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
    // JSON RULES
    fn write_project(&mut self, project: Project) {
        // crate::json::project::write_project(project);
        self.projects = NestedTableItems::<Project>::load();
    }

    pub fn write_close_new_project(&mut self, project: Project) {
        self.write_project(project);
        self.default_close();
    }

    fn write(&mut self) {
        crate::json::project::write_projects(&self.projects.items);
    }

    fn write_todo(&mut self) {
        let (i, mut project) = self.projects.current_state();
        let mut p = project.unwrap().clone();

        let current_todo = self.projects.current_todo();

        let todo = TodoBuilder::default()
            .id(0)
            // .todos
            // .iter()
            // .max_by_key(|p| p.id)
            // .and_then(|x| Some(x.id + 1))
            // .unwrap())
            .project_id(p.id)
            .parent_id(current_todo.map_or(1, |t| t.id))
            .depth(current_todo.map_or(1, |t| t.depth + 1))
            .todo(self.input.value().to_owned())
            .is_complete(false)
            .priority(1)
            .build()
            .unwrap();

        p.todos.push(todo);
        self.projects.items[i.unwrap()] = p;
        self.write();

        // RELOAD
        self.reload();
    }

    pub fn inc_todo_depth(&mut self, inc: i8) {
        let (i, mut project) = self.projects.current_state();
        let mut p = project.unwrap().clone();

        let mut current_todo = self.projects.current_todo().unwrap().clone();
        current_todo.change_depth(inc);
        // current_todo.map(|p| p.change_depth(inc));

        let idx = self.projects.substate.selected();
        p.todos[idx.unwrap()] = current_todo;

        self.projects.items[i.unwrap()] = p;
        self.write();

        // RELOAD
        self.reload();
    }

    fn update_project_desc(&mut self) {
        match self.projects.current_state() {
            (i, Some(p)) => {
                // crate::json::project::update_project_desc(p.id, self.input.value().to_owned());
                self.reload();
                self.projects.select_state(i);
            }
            _ => {}
        };
    }

    pub fn write_close_new_todo(&mut self) {
        self.write_todo();
        self.default_close();
    }

    pub fn write_close_description(&mut self) {
        self.update_project_desc();
        self.default_close();
    }

    pub fn write_close_gitconfig(&mut self) {
        // crate::json::tmp_config::write_tmp_to_project();
        // TODO reload app
        self.projects = NestedTableItems::<Project>::load();
        self.default_close();
    }

    pub fn write_close_new_category(&mut self) {
        // TODO delete

        match self.projects.current_state() {
            (i, Some(p)) => {
                let value = self.input.value().to_owned();
                // crate::json::category::write_category(&value);
                // TODO needs to work if above write is successful
                // crate::json::project::update_project_category(p, &value);
                // TODO reload
                self.projects = NestedTableItems::<Project>::load();
                self.projects.select_state(i);
            }
            _ => {}
        }
        self.default_close();
    }

    pub fn write_close_edit_category(&mut self) {
        // TODO delete
        match self.projects.current_state() {
            (i, Some(p)) => {
                let category = self.categories.current().unwrap().name.to_string();
                // TODO
                // crate::json::project::update_project_category(p, &category);
                self.projects = NestedTableItems::<Project>::load();
                self.projects.select_state(i);
            }
            _ => {}
        }
        self.default_close();
    }

    pub fn cycle_priority(&mut self) {
        //TODO delete
        // match self.projects.current_todos() {
        //     (idx, Some(t)) => {
        //         let priority = (t.priority + 1) % 3;
        //         crate::json::todo::update_todo_priority(t.id, priority);
        //         // TODO
        //     }
        //     _ => (),
    }

    pub fn delete_project(&mut self) {
        // TODO delete
        match self.projects.current() {
            Some(p) => {
                // crate::json::project::delete_project(p);
                self.projects = NestedTableItems::<Project>::load();
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
                    // update_project_sort(self.index, (p.sort + 1).into());
                    self.reload();
                    self.index = 0;
                    self.window.to_normal();
                }
                None => {}
            }
        }
    }
}
