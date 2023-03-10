use crate::other::viewer::{GitConfig, Project, Status};
use regex::{Captures, Regex};
use rusqlite::{params, Connection};
use wyd::{self, DATABASE};

use super::viewer::Todo;

/// CREATE TABLES
const CREATE_CONFIG: &str  
= "CREATE TABLE IF NOT EXISTS tmp_git_config (path varchar(255) not null primary key, is_selected tinyint(1) default 0, UNIQUE(path));";
const CREATE_PROJECT: &str 
= "CREATE TABLE IF NOT EXISTS project (id integer primary key autoincrement, path varchar(255), name varchar(255), cat varchar(255), status varchar(255), last_commit varchar(255));";
const CREATE_TODO: &str    
= "CREATE TABLE IF NOT EXISTS todo (id integer primary key autoincrement, parent_id integer, project_id integer, todo varchar(255), is_complete tinyint(1) default 0);";
pub fn initialize_db() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    // TODO I want to collect these errors
    conn.execute(CREATE_CONFIG, ())?;
    conn.execute(CREATE_PROJECT, ())?;
    conn.execute(CREATE_TODO, ()).expect("BBB!!");

    Ok(())
}

///
const READ_TMP: &str = "select path, is_selected from tmp_git_config where is_selected = 0 and path not in (select path from project)";
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

///
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
                // TODO is this the best way?
                last_commit: match row.get(5)? {
                    Some(x) => x,
                    None    => "N/A".to_string(),
                }
            })
        })
        .expect("A!!")
        .collect();

    res
}

/// TODOs
const READ_TODO: &str = "select id,parent_id,project_id,todo,is_complete from todo";
pub fn read_todo() -> Result<Vec<Todo>, rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

    let mut stmt = conn.prepare(READ_TODO)?;
    let res = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                project_id: row.get(2)?, 
                todo: row.get(3)?,
                is_complete: row.get(4)?
            })
        })
        .expect("A!!")
        .collect();

    res
}

const UPDATE_TODO: &str = "insert or replace into todo (id,parent_id,project_id,todo,is_complete) values (?1, ?2, ?3, ?4, ?5);";
pub fn update_todo(todo: &Todo) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;
    
    let mut write_stmt = conn.prepare(UPDATE_TODO)?;
    write_stmt.execute(
        params![
        todo.id,
        todo.parent_id,
        todo.project_id,
        todo.todo.as_str(),
        match todo.is_complete {
            true => true,
            _ => false,
        },
    ]).expect("AAA!");

    Ok(())
}

const WRITE_NEW_TODO: &str = "insert or replace into todo (parent_id,project_id,todo,is_complete) values (?1, ?2, ?3, ?4);";
pub fn write_new_todo(todos: Vec<Todo>) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;
    
    let mut write_stmt = conn.prepare(WRITE_NEW_TODO)?;
    for x in todos {
        write_stmt.execute(
            params![
            x.parent_id,
            x.project_id,
            x.todo.as_str(),
            match x.is_complete {
                true => true,
                _ => false,
            },
        ]).expect("AAA!");
    };

    Ok(())
}

///
const UPDATE_TMP: &str = "update tmp_git_config set is_selected = ?1 where path = ?2;";
pub fn update_tmp(tmp: &GitConfig) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(UPDATE_TMP, (&tmp.is_selected, &tmp.path))
        .expect("A");

    Ok(())
}

/// WRITE TEMPORARY PROJECTS
const INSERT_INTO_TMP: &str = "INSERT OR IGNORE INTO tmp_git_config (path) VALUES (?)";
pub fn write_tmp(tmp: Vec<String>) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut stmt = conn.prepare(INSERT_INTO_TMP)?;
    for x in tmp {
        stmt.execute([x])?;
    }

    Ok(())
}

/// all new tmps are written to
// const READ_TMP_SELECTED: &str    = "select path from tmp_git_config where is_selected = 1";
const READ_TMP_SELECTED: &str = "select path from tmp_git_config where is_selected = 1 and path not in (select path from project)";
const WRITE_TMP_TO_PROJECT: &str = "insert or replace into project (path,name,cat,status) values (?1, ?2, ?3, ?4);";
pub fn write_tmp_to_project() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut read_stmt = conn.prepare(READ_TMP_SELECTED)?;

    let tmp_project: Result<Vec<Project>, rusqlite::Error> = read_stmt
        .query_map([], |row| 
            {Ok(Project {
                id: 0,
                path: row.get(0)?,
                name: regex_repo(row.get(0)?),
                category: "C".to_owned(),
                status: Status::Unstable,
                last_commit: "N/A".to_owned(),
            })}
        )
        .expect("AAAA")
        .collect();
    
    let mut write_stmt = conn.prepare(WRITE_TMP_TO_PROJECT)?;
    for x in tmp_project? {
        write_stmt.execute([
            x.path,
            x.name,
            x.category,
            x.status.to_string(),
        ]).expect("AAA!");
    };

    Ok(())
}

/// 
fn regex_repo(path: String) -> String {
    let re = Regex::new(r"(.+)/([^/]+)").expect("AAAA");
    let caps = re.captures(&path).unwrap();

    caps.get(0).unwrap().as_str().to_string()
}
