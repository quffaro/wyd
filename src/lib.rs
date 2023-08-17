#![allow(unused_imports)]
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

/// JSON
pub mod json;

/// Request
pub mod request;

const CONFIG: &str = "/.config/wyd/";
const PAT: &str = "/.config/wyd/pat.txt";
const DB: &str = "wyd.db";
const WYD_CONFIG: &str = "config";

pub const PATH_PAT: &str = formatcp!("{}{}", CONFIG, PAT);
pub const PATH_DB: &str = formatcp!("{}{}", CONFIG, DB);
pub const PATH_CONFIG: &str = formatcp!("{}{}", CONFIG, WYD_CONFIG);

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
