use const_format::formatcp;
use dirs::home_dir;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Terminal user interface.
pub mod tui;

/// Event handler.
// TODO
// pub mod handler;

/// Request
pub mod request;

const CONFIG: &str = "/.config/wyd/";

pub const GITCONFIG_SUFFIX: &str = ".git/config";
pub const GLOB_GITCONFIG_SUFFIX: &str = formatcp!("**/{}", GITCONFIG_SUFFIX);
pub const CONFIG_SEARCH_FOLDER: &str = "~/Documents/";

pub fn home_path(path: &str) -> String {
    format!(
        "{}{}",
        home_dir().unwrap().into_os_string().into_string().unwrap(),
        path.to_owned()
    )
}
