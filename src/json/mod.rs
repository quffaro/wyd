use crate::app::structs::projects::Project;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

pub mod category;
pub mod project;
pub mod tmp_config;
pub mod todo;

/// the idea is to have a single json object with
///     categories
///     projects
///         todos

const JSON_DB: &str = "./config/wyd/db.json";

// TODO to may?
pub fn read_json() -> Result<Vec<Project>> {
    let mut projects: Vec<Project> = {
        let data = fs::read_to_string(JSON_DB);
        match data {
            Ok(d) => serde_json::from_str(&d).unwrap(),
            Err(e) => {
                let new = json!(
                    {"projects": []});
                fs::write(JSON_DB, serde_json::to_string_pretty(&new).unwrap());
                vec![]
            }
        }
    };

    projects
}

pub fn write_json(projects: Vec<Project>) -> Result<()> {
    Ok(())
}
