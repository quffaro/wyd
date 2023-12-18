use self::structs::{
    // category::Category,
    // config::{load_config, wyd_to_color, Config},
    focus::{Focus, WindowBase, WindowPopup, WindowTab},
    // gitconfig::GitConfig,
    items::Item,
    jobs::{JobRoster, LoadingState},
    ListNav,
    TableItems,
};
use crate::app::ui::{
    base::{
        render_projects, render_title, render_title_and_search, render_todo, render_todo_and_desc,
    },
    popup::render_popup_todo,
    render_loading,
};
use crate::home_path;
use derive_builder::Builder;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::terminal::Frame;
use std::error;
use tui_input::Input;

pub mod regex;
pub mod structs;
pub mod ui;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application
#[derive(Builder, Debug)]
pub struct App {
    pub running: bool,
    pub tick: u8, // TODO cyclic?
    pub index: u8,
    pub input: Input,
    pub buffer: Vec<u8>, // TODO trait for types which can be yanked to buffer?
    pub jobs: JobRoster,
    pub focus: Focus,
    pub config: Vec<u8>,
    pub data: TableItems<Item>,
}

// TODO builder
impl Default for App {
    fn default() -> Self {
        App {
            running: true,
            tick: 0,
            index: 0,
            input: Input::default(),
            buffer: vec![],
            jobs: JobRoster::default(),
            focus: Focus::default(),
            config: vec![],
            data: TableItems::<Item>::default(),
        }
    }
}

impl App {
    /// Loads data into the app
    pub fn load(&mut self) {
        // self.data = TableItems<Item>::load();
    }
    /// Saves app data
    pub fn save(&mut self) {}
    /// handles tick event of the terminal
    pub fn tick(&mut self) {
        self.tick = (self.tick + 1) % 5;
    }
    ///
    // TODO can data be Algebraic so we can use just one next/previous with +/- 1?
    pub fn next(&mut self) {}
    pub fn previous(&mut self) {}
    pub fn cycle_focus(&mut self) {}
    pub fn toggle(&mut self) {}
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let size = frame.size();

        // vertical layout
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

        // chunk 0
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
        frame.render_stateful_widget(projects, chunks[1], &mut self.data.state);

        // CHUNK 2
        let project_info_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);

        let (todo_block, desc_block) = render_todo_and_desc(self);
        let todo_table = render_todo(self);
        frame.render_stateful_widget(todo_table, project_info_chunk[0], &mut self.data.state);
        frame.render_widget(desc_block, project_info_chunk[1]);

        // CHUNK 3
        let loading = render_loading(self);
        frame.render_widget(loading, chunks[3]);
    }

    /// Yanks current item into buffer
    pub fn yank(&mut self) {}
    /// Pastes current item from buffer
    pub fn paste(&mut self) {}
}
