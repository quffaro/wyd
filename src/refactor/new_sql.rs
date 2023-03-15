use crate::refactor::new_lib::DATABASE;
use crate::refactor::new_lib::{GitConfig, Project, Todo};
use rusqlite::{params, Connection, Result};

/// CREATE TABLES
const CREATE_CONFIG: &str = "CREATE TABLE IF NOT EXISTS tmp_git_config (
    path        varchar(255) not null primary key, 
    is_selected tinyint(1) default 0, 
    UNIQUE(path)
);";
const CREATE_PROJECT: &str = "CREATE TABLE IF NOT EXISTS project (
    id          integer primary key autoincrement, 
    path        varchar(255), 
    name        varchar(255), 
    desc        varchar(4000), 
    cat         varchar(255), 
    status      varchar(255),
    is_git      tinyint(1),
    last_commit varchar(255),
    UNIQUE(path)
);";
const CREATE_TODO: &str = "CREATE TABLE IF NOT EXISTS todo (
    id          integer primary key autoincrement,
    parent_id   integer,
    project_id  integer,
    todo        varchar(255),
    is_complete tinyint(1) default 0
);";

pub fn initialize_db() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(CREATE_CONFIG, ())?;
    conn.execute(CREATE_PROJECT, ())?;
    conn.execute(CREATE_TODO, ())?;

    Ok(())
}

/// READ PROJECTS FROM DB
const READ_PROJECT: &str = "select id,path,name,desc,cat,status,is_git,last_commit from project";
pub fn read_project(conn: Option<Connection>) -> Result<Vec<Project>, rusqlite::Error> {
    let conn = match conn {
        Some(c) => c,
        None => Connection::open(wyd::DATABASE)?,
    };

    let mut stmt = conn.prepare(READ_PROJECT)?;
    let res = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                desc: match row.get(3)? {
                    Some(x) => x,
                    None => "N/A".to_string(),
                },
                category: row.get(4)?,
                status: row.get(5)?, // TODO is this the best way?
                is_git: row.get(6)?,
                last_commit: match row.get(7)? {
                    Some(x) => x,
                    None => "N/A".to_string(),
                },
            })
        })
        .expect("A!!")
        .collect();

    res
}

/// TODOs
const READ_TODO: &str = "select id,parent_id,project_id,todo,is_complete from todo";
pub fn read_todo(conn: Option<Connection>) -> Result<Vec<Todo>, rusqlite::Error> {
    let conn = match conn {
        Some(c) => c,
        None => Connection::open(wyd::DATABASE)?,
    };

    let mut stmt = conn.prepare(READ_TODO)?;
    let res = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                project_id: row.get(2)?,
                todo: row.get(3)?,
                is_complete: row.get(4)?,
            })
        })
        .expect("A!!")
        .collect();

    res
}

///
const READ_TMP: &str = "select path, is_selected from tmp_git_config where is_selected = 0 and path not in (select path from project)";
pub fn read_tmp(conn: Option<Connection>) -> Result<Vec<GitConfig>, rusqlite::Error> {
    let conn = match conn {
        Some(c) => c,
        None => Connection::open(wyd::DATABASE)?,
    };

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

/// WRITE PROJECT TO DB
const INSERT_PROJECT: &str = "insert or replace into project (
    path,
    name, 
    desc, 
    cat, 
    status, 
    is_git, 
    last_commit
) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
pub fn write_project(project: Project) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut stmt = conn.prepare(INSERT_PROJECT)?;
    stmt.execute(params![
        project.path,
        project.name,
        project.desc,
        project.category.to_string(),
        project.status.to_string(),
        project.is_git,
        project.last_commit,
    ]);

    Ok(())
}
