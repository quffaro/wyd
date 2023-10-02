use crate::app::structs::projects::{Project, ProjectBuilder};
use crate::app::App;
use crate::json::write_json;
use serde::Deserialize;
use std::env;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// read
// TODO
// pub fn read_projects() -> Result<Project, Box<dyn Error>> {
//     ProjectBuilder::default().build()
// }

pub fn write_projects(projects: &Vec<Project>) -> Result<(), Box<dyn Error>> {
    Ok(write_json(&projects)?)
}

// pub fn add_project_in_dir(app: &mut App, project: Option<Project>) -> Result<(), Box<dyn Error>> {
//     let copy = &app.projects.items;
//     match project {
//         Some(p) => write_projects(copy.push(p)),
//         None => Ok(()),
//     }
//     // Ok(ProjectBuilder::new().build())
// }
