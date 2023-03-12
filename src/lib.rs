use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use std::fmt;
use std::str::FromStr;
use strum_macros::EnumString;
use tui::style::Color;

/// SQL
pub const DATABASE: &str = "projects.db";
pub const SEARCH_DIRECTORY_PREFIX: &str = "/home/cuffaro/Documents"; // CUFFARO IS NOT GUARANTEED!
pub const CONFIG_PATH_SUFFIX: &str = "**/.git/config";
pub const CONFIG_SEARCH_PREFIX: &str = "~/Documents/";

/// WINDOWS
pub const WINDOW_PROJECTS: &str = "projects";
pub const WINDOW_TODO: &str = "todo";
pub const WINDOW_DESCRIPTION: &str = "description";
pub const WINDOW_POPUP_CONFIGS: &str = "configs";
pub const WINDOW_POPUP_ADD_TODO: &str = "add-todo";
pub const WINDOW_POPUP_EDIT: &str = "edit";
pub const WINDOW_POPUP_DESC: &str = "desc";

#[derive(PartialEq, Eq, Debug, Clone, EnumString)]
pub enum Status {
    Stable,
    Unstable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
// TODO refactor to include-sql
impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse::<Status>()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    fn help_message(&self) -> &'static str {
        match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Insert => "type Esc to back to normal mode",
        }
    }

    fn cursor_color(&self) -> Color {
        match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum WindowStatus {
    Loaded,
    NotLoaded,
}
