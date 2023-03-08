use crate::other::viewer::{GitConfig, Project, Status};
use rusqlite::Connection;
use wyd::{self, DATABASE};
use regex::{Captures, Regex};

/// CREATE TABLES
const CREATE_CONFIG: &str = "CREATE TABLE IF NOT EXISTS tmp_git_config (path varchar(255) not null primary key, is_selected tinyint(1) default 0, UNIQUE(path));";
const CREATE_PROJECT: &str = "CREATE TABLE IF NOT EXISTS project (id integer primary key autoincrement, path varchar(255), name varchar(255), cat varchar(255), status varchar(255), last_commit varchar(255),);";
const CREATE_TODO: &str = "CREATE TABLE IF NOT EXISTS todo (id integer primary key autoincrement, project_id int, todo varchar(255);";
pub fn initialize_db() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    // TODO I want to collect these errors
    conn.execute(CREATE_CONFIG, (),);
    conn.execute(CREATE_PROJECT, (),);
    conn.execute(CREATE_TODO, (),);

    Ok(())
}

//
const READ_TMP: &str = "select path, is_selected from tmp_git_config where is_selected = 0";
pub fn read_tmp() -> Result<Vec<GitConfig>, rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

    let mut stmt = conn.prepare(READ_TMP)?;
    let res = stmt
        .query_map([], |row| {
            Ok(GitConfig {
                path: row.get(0)?,
                is_selected: row.get(1)?,
            })
        })?
        .collect();

    res
}

const READ_PROJECT: &str = "select id,path,name,cat,status,last_commit from project";
pub fn read_project() -> Result<Vec<Project>, rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

    let mut stmt = conn.prepare(READ_PROJECT)?;
    let res = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                category: row.get(3)?,
                status: Status::Stable,
                // match row.get(4)? {
                //     "Stable" => Status::Stable,
                //     "Unstable" => Status::Unstable,
                //     _ => Status::Unstable,
                // },
                last_commit: row.get(5)?,
            })
        })
        .expect("A!!")
        .collect();

    // println!("{:#?}", res);

    res
}

const UPDATE_TMP: &str = "update tmp_git_config set is_selected = ?1 where path = ?2;";
pub fn update_tmp(tmp: &GitConfig) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(
        UPDATE_TMP,
        (&tmp.is_selected, &tmp.path),
    )
    .expect("A");

    Ok(())
}

/// WRITE TEMPORARY PROJECTS
const INSERT_INTO_TMP: &str = "INSERT INTO tmp_git_config (path) VALUES (?)";
pub fn write_tmp(tmp: Vec<String>) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut stmt = conn.prepare(INSERT_INTO_TMP)?;
    for x in tmp {
        stmt.execute([x])?;
    }

    Ok(())
}

// all new tmps are written to
const READ_TMP_SELECTED: &str = "select path from tmp_git_config where is_selected = 1";
const WRITE_TMP_TO_PROJECT: &str = "insert or replace into project (path,name,cat,status) values (?1, ?2, ?3, ?4);";
pub fn write_tmp_to_project() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;
    
    let mut read_stmt = conn.prepare(READ_TMP_SELECTED)?;
    let mut write_stmt = conn.prepare(
        WRITE_TMP_TO_PROJECT,
    )?;

    let re = Regex::new(r#"/^(.+)\/([^\/]+)$/gm"#).unwrap();
    read_stmt
        .query_map([], |row| {
            // let path = row.get_unwrap(0);
            let path = "E";
            let caps= re.captures(path).unwrap();
            write_stmt.execute([
                "A".to_owned(),
                "B".to_owned(),
                "C".to_owned(),
                "D".to_owned(),
            ]).expect("WRITE ERROR");
            Ok(())
        }).expect("READ_ERROR")
        .collect();

    

    

    // for tmp in &tmp_to_write {
    //     let caps = re.captures(&tmp).unwrap();
    //     stmt.execute([
    //         &tmp,
    //         &tmp,
    //         &"Unknown".to_owned(),
    //         &"Unstable".to_owned(),
    //     ]).expect("A!!!");
    // }

    // for x in &tmp {
    // TODO get name of parent directory
    // let caps = re.captures(&tmp.path).unwrap();

    // stmt.execute([
    //     &tmp.path,
    //     &tmp.path,
    //     // &caps.get(1).map_or("", |m| m.as_str()).to_owned(),
    //     // .map_or(&"".to_owned(), |m| &m.as_str().to_string()),
    //     &"Unknown".to_owned(),
    //     &"Unstable".to_owned(),
    // ])
    // .expect("A");
    // }
    // println!("HELLO");

    Ok(())
}
