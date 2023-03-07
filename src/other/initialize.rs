// look through ~ and find .git files. stop recursing that directory when a .git file is found and
// return its path
use glob::{glob};
use rusqlite::{Connection, Result};
use shellexpand;
use std::path::PathBuf;
use wyd::{CONFIG_SEARCH_PREFIX, CONFIG_PATH_SUFFIX, DATABASE};

// TODO need to find config files _not in projects._ We'll need our own table
pub fn initialize() -> Result<()> {
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
        // .map(|x| to_tgp(x))
        .collect();

    // tmp
    write_tmp(tmp)
}

// fn tmp_git_config
fn write_tmp(tmp: Vec<String>) -> Result<()> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(
        "create table if not exists tmp_git_config (path varchar(255) not null primary key, is_selected tinyint(1) default 0);",
        (),
    )
    .expect("Failed");

    let mut stmt = conn.prepare("INSERT INTO tmp_git_config (path) VALUES (?)")?;
    for x in tmp {
        stmt.execute([x])?;
    }

    Ok(())
}

