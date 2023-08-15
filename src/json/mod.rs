use serde::{Deserialize, Serialize};

pub mod category;
pub mod project;
pub mod tmp_config;
pub mod todo;

/// the idea is to have a single json object with 
///     categories
///     projects
///         todos

const DB:= &str "./config/wyd/db.json" ;

pub fn read_json() -> {} {
        let file_content = fs::read_to_string(DB).expect("error reading file");
        serde_json::from_str::<Value>(&file_content).expect("error serializing to JSON")
    };


pub fn initialize_db() -> Result<()> {
    // create file 
    match fs::read_to_string(DB)
        Ok(_) => Ok(())
        Err(e) => {
            let new_db = object!{
                categories: [],
                projects: []
            }
        }
}
