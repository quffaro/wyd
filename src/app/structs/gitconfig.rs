use super::super::regex::regex_last_dir;
use super::{TableItems, TableState};
use crate::sql::tmp_config::{read_tmp, update_tmp};
use crate::{CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX, GLOB_GITCONFIG_SUFFIX};
use glob::glob;
use ini::Ini;
use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub path: String,
    pub is_selected: bool,
}

impl GitConfig {
    pub fn load(conn: &Connection) -> Vec<GitConfig> {
        // TODO unwrap
        read_tmp(conn).unwrap()
        // vec![]
    }
}

impl TableItems<GitConfig> {
    pub fn load(conn: &Connection) -> TableItems<GitConfig> {
        TableItems {
            items: GitConfig::load(conn),
            state: TableState::default(),
        }
    }
    pub fn toggle(&mut self, conn: &Connection) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].is_selected = !self.items[i].is_selected;
                update_tmp(conn, &self.items[i]); // TODO
            } else {
                continue;
            }
        }
    }
}

pub fn read_git_config(path: String) -> String {
    let config_path = format!("{}{}", path, ".git/config").to_owned();
    // dbg!(&config_path);
    let config = Ini::load_from_file(config_path).expect(path.as_str());
    let url = config
        .get_from(Some("remote \"origin\""), "url")
        .unwrap()
        .to_owned();

    url
}

pub fn guess_git_owner(path: String) -> String {
    let url = read_git_config(path);
    let result = regex_last_dir(url);
    // dbg!(&result);
    result
}

pub fn fetch_config_files() -> Vec<String> {
    let expanded_path = shellexpand::tilde(CONFIG_SEARCH_FOLDER);
    let pattern: PathBuf = [&expanded_path, GLOB_GITCONFIG_SUFFIX].iter().collect();

    let tmp: Vec<String> = glob(pattern.to_str().unwrap())
        .expect("expectation!!")
        .filter_map(|x| x.ok())
        .map(|x| {
            x.into_os_string()
                .into_string()
                .unwrap()
                .replace(GITCONFIG_SUFFIX, "")
        })
        .collect();

    tmp
}
