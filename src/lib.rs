use const_format::formatcp;
use dirs::home_dir;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// SQL scripts
pub mod sql;

pub const CONFIG: &str = "/.config/wyd/";
pub const PAT: &str = "pat.txt";
pub const DB: &str = "wyd.db";
pub const PATH_PAT: &str = formatcp!("{}{}", CONFIG, PAT);
pub const PATH_DB: &str = formatcp!("{}{}", CONFIG, DB);

pub const GITCONFIG_SUFFIX: &str = ".git/config";
pub const GLOB_GITCONFIG_SUFFIX: &str = formatcp!("**/{}", GITCONFIG_SUFFIX);
pub const CONFIG_SEARCH_FOLDER: &str = "/Documents/";

pub fn home_path(path: &str) -> String {
    format!(
        "{}{}",
        home_dir().unwrap().into_os_string().into_string().unwrap(),
        path.to_owned()
    )
}
