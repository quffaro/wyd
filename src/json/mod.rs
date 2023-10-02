use crate::app::structs::category::Category;
use crate::app::structs::projects::Project;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod category;
pub mod project;
// pub mod todo;
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFile {
    pub categories: Vec<Category>,
    pub projects: Vec<Project>,
}

impl SaveFile {
    fn new() -> Self {
        SaveFile {
            projects: vec![],
            categories: vec![],
        }
    }
}

/// the idea is to have a single json object with
///     categories
///     projects
///         todos

const JSON_DB: &str = "db.json";
// "~./config/wyd/db.json";

// read
// TODO to may?
pub fn read_json() -> Result<Vec<Project>, Box<dyn Error>> {
    let data = fs::read_to_string(JSON_DB);
    let result: Vec<Project> = match fs::read_to_string(JSON_DB) {
        Ok(d) => serde_json::from_str(&d)?,
        Err(e) => {
            let new = vec![];
            write_json(&new);
            new
        }
    };

    Ok(result)
}

pub fn write_json(db: &Vec<Project>) -> Result<(), std::io::Error> {
    fs::write(JSON_DB, serde_json::to_string_pretty(db).unwrap())
}
