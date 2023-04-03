use crate::{home_path, PATH_CONFIG};
use ini::Ini;
use std::string::ToString;
use std::{fmt, str::FromStr};
use strum_macros::Display;
// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

#[derive(Clone, Debug, Display)]
pub enum WydColor {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
    Reset,
}

impl FromStr for WydColor {
    type Err = ();

    fn from_str(input: &str) -> Result<WydColor, Self::Err> {
        match input {
            "Red" => Ok(WydColor::Red),
            "Yellow" => Ok(WydColor::Yellow),
            "Green" => Ok(WydColor::Green),
            "Blue" => Ok(WydColor::Blue),
            "Black" => Ok(WydColor::Black),
            "Reset" => Ok(WydColor::Reset),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorConfig {
    pub bd: WydColor,
    pub hl: WydColor,
    pub bg: WydColor,
}

impl ColorConfig {
    pub fn default() -> ColorConfig {
        Self {
            bd: WydColor::Yellow,
            hl: WydColor::Yellow,
            bg: WydColor::Reset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub owner: String,
    pub search_folder: String,
    pub db: String,
    pub color: ColorConfig,
}

pub fn get_config_color(config: Ini) -> ColorConfig {
    ColorConfig {
        bd: WydColor::from_str(
            config
                .get_from(Some("color"), "bd")
                .ok_or("Yellow")
                .unwrap(),
        )
        .unwrap(),
        hl: WydColor::from_str(
            config
                .get_from(Some("color"), "hl")
                .ok_or("Yellow")
                .unwrap(),
        )
        .unwrap(),
        bg: WydColor::from_str(
            config
                .get_from(Some("color"), "bg")
                .ok_or("Yellow")
                .unwrap(),
        )
        .unwrap(),
    }
}

pub fn load_config() -> Option<Config> {
    match Ini::load_from_file(home_path(PATH_CONFIG)) {
        Ok(config) => Some(Config {
            owner: config.get_from(Some("user"), "owner").unwrap().to_owned(),
            search_folder: config
                .get_from(Some("user"), "search_folder")
                .unwrap()
                .to_owned(),
            db: config.get_from(Some("user"), "db").unwrap().to_owned(),
            color: get_config_color(config),
        }),
        Err(_) => None,
    }
}

pub fn init_config(config: Config) {
    let mut conf = Ini::new();
    conf.with_section(Some("user"))
        .set("owner", config.owner)
        .set("search_folder", config.search_folder)
        .set("db", config.db);
    conf.with_section(Some("color"))
        .set("bg", config.color.bg.to_string())
        .set("hl", config.color.hl.to_string())
        .set("bd", config.color.bd.to_string());

    conf.write_to_file(home_path(PATH_CONFIG)).unwrap();
}
