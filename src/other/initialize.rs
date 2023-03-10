// look through ~ and find .git files. stop recursing that directory when a .git file is found and
// return its path
use glob::{glob};
// use rusqlite::{Result};
use shellexpand;
use std::path::PathBuf;
use wyd::{CONFIG_SEARCH_PREFIX, CONFIG_PATH_SUFFIX};
use crate::other::sql::{initialize_db, write_tmp};

// TODO need to find config files _not in projects._ We'll need our own table
pub fn initialize() -> Result<(), rusqlite::Error> {

    // CREATE DATABASE
    initialize_db()?;

    // FIND CONFIG FILES
    // TODO which are not in project table already...
    let tmp = fetch_config_files();

    // tmp
    write_tmp(tmp)
}

fn fetch_config_files() -> Vec<String> {
    let expanded_path = shellexpand::tilde(CONFIG_SEARCH_PREFIX);
    let pattern: PathBuf = [&expanded_path, CONFIG_PATH_SUFFIX].iter().collect();

    let tmp: Vec<String> = glob(pattern.to_str().unwrap())
        .expect("expectation!!")
        .filter_map(|x| x.ok())
        .map(|x| {
            x.into_os_string()
                .into_string()
                .unwrap()
                .replace(CONFIG_PATH_SUFFIX, "")
        })
        .collect();

    tmp
}

// fn tmp_git_config


