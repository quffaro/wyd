use crate::app::structs::config::load_config;

// https://applied-math-coding.medium.com/
// use std::cell::RefCell;
// use std::rc::Rc;
use super::{ListNav, ListState, PlainListItems};
use ratatui::style::Color;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub base: BaseWindow,
    pub popup: Popup,
    pub status: WindowStatus,
    pub mode: Mode,
}

impl Window {
    pub fn load(config: bool) -> Window {
        Window {
            base: BaseWindow::Project,
            popup: match config {
                true => Popup::None,
                false => Popup::Config,
            },
            status: WindowStatus::NotLoaded,
            mode: Mode::Normal,
        }
    }
    pub fn mode_color(&self) -> Color {
        match self.mode {
            Mode::Insert => Color::Yellow,
            Mode::Normal => Color::Green,
        }
    }
    pub fn to_normal(&mut self) {
        self.mode = Mode::Normal
    }
    pub fn to_insert(&mut self) {
        self.mode = Mode::Insert
    }
    /// WINDOW RULES
    pub fn popup(&mut self, popup: Popup, mode: Option<Mode>) {
        self.popup = popup;
        match mode {
            Some(m) => self.mode = m,
            None => (),
        }
    }
    pub fn close_popup(&mut self) {
        self.popup = Popup::None;
        self.status = WindowStatus::NotLoaded;
        self.mode = Mode::Normal;
    }
    pub fn base_focus_color(&self) -> Color {
        match self {
            Window {
                popup: Popup::None,
                base: _,
                ..
            } => Color::Yellow,
            _ => Color::White,
        }
    }
    fn _to_project() -> Window {
        Window {
            base: BaseWindow::Project,
            popup: Popup::None,
            status: WindowStatus::NotLoaded,
            mode: Mode::Normal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, EnumString, EnumIter)]
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

impl PlainListItems<BaseWindow> {
    fn _new() -> PlainListItems<BaseWindow> {
        PlainListItems {
            items: BaseWindow::iter().collect(),
            state: ListState::default(),
        }
    }
    pub fn current(&mut self) -> Option<&BaseWindow> {
        let idx = self.get_state_selected().unwrap();
        self.items.iter().nth(idx)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Popup {
    None,
    SearchGitConfigs,
    AddTodo,
    ReadTodo,
    EditCat,
    NewCat,
    EditDesc,
    Help,
    Config,
    DeleteProject,
    NewProject,
}

impl fmt::Display for Popup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::None => write!(f, "NO POPUP"),
            Self::SearchGitConfigs => write!(f, "SEARCH PROJECTS"),
            Self::AddTodo => write!(f, "ADD TODO"),
            Self::ReadTodo => write!(f, "READ TODO"),
            Self::EditCat => write!(f, "EDIT CATEGORY"),
            Self::NewCat => write!(f, "NEW CATEGORY"),
            Self::EditDesc => write!(f, "EDIT DESCRIPTION"),
            Self::Help => write!(f, "HELP"),
            Self::Config => write!(f, "CONFIG"),
            Self::DeleteProject => write!(f, "DELETE PROJECT"),
            Self::NewProject => write!(f, "NEW PROJECT"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowStatus {
    Loaded,
    NotLoaded,
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
