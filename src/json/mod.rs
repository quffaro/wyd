use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod category;
pub mod project;
pub mod tmp_config;
pub mod todo;

/// the idea is to have a single json object with
///     categories
///     projects
///         todos

const JSON_DB: &str = "./config/wyd/db.json";

pub fn read_json() -> String {
    let file_content = fs::read_to_string(JSON_DB).expect("error reading file");
    serde_json::from_str::<Value>(&file_content).expect("error serializing to JSON")
}

pub fn initialize_db() -> Result<()> {
    // create file
    Ok()
    // match fs::read_to_string(JSON_DB)
    //     Ok(_) => Ok(())
    //     Err(e) => {
    //         let new_db = object!{
    //             categories: [],
    //             projects: []
    //         }
    //     }
}
