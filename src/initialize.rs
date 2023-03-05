// look through ~ and find .git files. stop recursing that directory when a .git file is found and
// return its path
use glob::{glob, Paths, PatternError};
use rusqlite::{Connection, Result};
use shellexpand;
use std::{env, io, path::PathBuf};

// #[derive(Clone, Debug)]
// struct TmpGitPath {
//     path: String,
//     is_selected: bool,
// }

// fn to_tgp(x: PathBuf) -> TmpGitPath {
//     TmpGitPath {
//         id: 0,
//         path: x.into_os_string().into_string().unwrap(),
//     }
// }

// TODO need to find config files _not in projects._ We'll need our own table
pub fn initialize() -> Result<()> {
    let expanded_path = shellexpand::tilde("~/Documents/");
    let pattern: PathBuf = [&expanded_path, "**/.git/config"].iter().collect();

    let tmp: Vec<String> = glob(pattern.to_str().unwrap())
        .expect("expectation!!")
        .filter_map(|x| x.ok())
        .map(|x| {
            x.into_os_string()
                .into_string()
                .unwrap()
                .replace("/.git/config", "")
        })
        // .map(|x| to_tgp(x))
        .collect();

    // tmp
    write_tmp(tmp)
}

// fn tmp_git_config
fn write_tmp(tmp: Vec<String>) -> Result<()> {
    let conn = Connection::open("projects.db")?;

    conn.execute(
        "create table if not exists tmp_git_config (path varchar(255) not null primary key, is_selected tinyint(1) default 0);",
        (),
    )
    .expect("Failed");

    // for x in tmp {
    //     conn.execute(
    //         "INSERT INTO tmp_git_config (path, is_selected) VALUES (?1, ?2)",
    //         (&x.path, &x.is_selected.to_string()),
    //     )
    // }
    let mut stmt = conn.prepare("INSERT INTO tmp_git_config (path) VALUES (?)")?;
    for x in tmp {
        stmt.execute([x])?;
    }

    Ok(())
}

// fn read_tmp() -> Result<Vec<TmpGitPath>> {
//     let conn = Connection::open("projects.db")?;

//     let mut stmt = conn.prepare("SELECT path FROM tmp_git_config")?;
//     let tgp_iter = stmt.query_map([], |row| {
//         Ok(TmpGitPath {
//             id: row.get(0)?,
//             path: row.get(1)?,
//         })
//     })?;

//     tgp_iter
// }
