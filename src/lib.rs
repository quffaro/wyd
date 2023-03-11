use std::fmt;

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Status {
    Stable,
    Unstable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum WindowStatus {
    Loaded,
    NotLoaded,
}
