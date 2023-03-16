// look through ~ and find .git files. stop recursing that directory when a .git file is found and
// return its path

use crate::other::sql::{initialize_db, write_tmp};
use glob::glob;
use shellexpand;
use std::path::PathBuf;

use wyd::{CONFIG_PATH_SUFFIX, CONFIG_SEARCH_PREFIX};

// TODO need to find config files _not in projects._ We'll need our own table
// #[tokio::main]
pub fn initialize() -> Result<(), rusqlite::Error> {
// pub async fn initialize() -> Result<(), rusqlite::Error> {
    // CREATE DATABASE
    initialize_db()?;

    // TODO: this is not best practice according to tokio. This should also be a true asynchronous
    // process, but our viewer still awaits this program. Initialize should be moved viewer and
    // accept App so it can call App methods.
    // tokio::task::spawn_blocking(|| {
    //     request_string();
    //     // println!("Bingus");
    // })
    // .await
    // .unwrap();

    // FIND CONFIG FILES
    let tmp = fetch_config_files();
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
