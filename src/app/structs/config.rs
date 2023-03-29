
use ini::Ini; 
use crate::{PATH_CONFIG, home_path};

#[derive(Debug)]
pub struct Config {
    pub owner: String,
    pub search_folder: String,
    pub db: String,
}


pub fn load_config() -> Option<Config> {
    match Ini::load_from_file(home_path(PATH_CONFIG)) {
        Ok(config) => Some(
            Config { 
                owner: config.get_from(Some("user"), "owner").unwrap().to_owned(), 
                search_folder: config.get_from(Some("user"), "search_folder").unwrap().to_owned(), 
                db: config.get_from(Some("user"), "db").unwrap().to_owned() 
            }
        ),
        Err(_)     => None,
    }
}

pub fn init_config(config: Config) {
    let mut conf = Ini::new();
    conf.with_section(Some("user"))
        .set("owner", config.owner)
        .set("search_folder", config.search_folder)
        .set("db", config.db);

    conf.write_to_file(home_path(PATH_CONFIG)).unwrap();
}