use super::structs::{GitConfig, Project, Todo};
use regex::Regex;
use rusqlite::{params, Connection, Result};
use wyd::{self, Category, Status, DATABASE};

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
    // TODO this needs to happen in a target library
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
// TODO not calling status
const READ_PROJECT: &str = "select id,path,name,desc,cat,status,is_git,last_commit from project";
pub fn read_project() -> Result<Vec<Project>, rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

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

///
const UPDATE_PROJECT_DESC: &str = "update project set desc = ?1 where id = ?2;";
pub fn update_project_desc(project: &Project, desc: String) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(UPDATE_PROJECT_DESC, (desc, &project.id))
        .expect("A");

    Ok(())
}

const UPDATE_PROJECT_CAT: &str = "update project set cat = ?1 where id = ?2;";
pub fn update_project_category(project: &Project, cat: &Category) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(UPDATE_PROJECT_CAT, (format!("{}", cat), project.id))
        .expect("A");

    Ok(())
}

const UPDATE_PROJECT_STATUS: &str = "update project set status = ?1 where id = ?2;";
pub fn update_project_status(project: &Project) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(
        UPDATE_PROJECT_STATUS,
        (format!("{}", project.status), project.id),
    )
    .expect("A");

    Ok(())
}

const UPDATE_PROJECT_LAST_COMMIT: &str = "update project set last_commit = ?1;";
pub fn update_project_last_commit(last_commit: String) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(UPDATE_PROJECT_LAST_COMMIT, params![last_commit])
        .expect("AAA");

    Ok(())
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
                is_complete: row.get(4)?,
            })
        })
        .expect("A!!")
        .collect();

    res
}

const WRITE_PROJECT: &str = "insert or replace into project (
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

    let mut stmt = conn.prepare(WRITE_PROJECT)?;
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

const UPDATE_TODO: &str = "insert or replace into todo (
    id,
    parent_id,
    project_id,
    todo,
    is_complete
) values (?1, ?2, ?3, ?4, ?5);";
pub fn update_todo(todo: &Todo) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut write_stmt = conn.prepare(UPDATE_TODO)?;
    write_stmt
        .execute(params![
            todo.id,
            todo.parent_id,
            todo.project_id,
            todo.todo.as_str(),
            match todo.is_complete {
                true => true,
                _ => false,
            },
        ])
        .expect("AAA!");

    Ok(())
}

/// TODOs
const DELETE_TODO: &str = "delete from todo where id = ?1;";
pub fn db_delete_todo(id: u8) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

    conn.execute(DELETE_TODO, params![id]).expect("A");

    Ok(())
}
const WRITE_NEW_TODO: &str =
    "insert or replace into todo (
        parent_id,
        project_id,
        todo,
        is_complete
) values (?1, ?2, ?3, ?4);";
pub fn write_new_todo(todos: Vec<Todo>) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut write_stmt = conn.prepare(WRITE_NEW_TODO)?;
    for x in todos {
        write_stmt
            .execute(params![
                x.parent_id,
                x.project_id,
                x.todo.as_str(),
                match x.is_complete {
                    true => true,
                    _ => false,
                },
            ])
            .expect("AAA!");
    }

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
const WRITE_TMP_TO_PROJECT: &str =
    "insert or replace into project (path,name,cat,status,is_git) values (?1, ?2, ?3, ?4,1);";
pub fn write_tmp_to_project() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    let mut read_stmt = conn.prepare(READ_TMP_SELECTED)?;

    let tmp_project: Result<Vec<Project>, rusqlite::Error> = read_stmt
        .query_map([], |row| {
            Ok(Project {
                id: 0,
                path: row.get(0)?,
                name: regex_repo(row.get(0)?),
                desc: "Example".to_owned(),
                category: Category::Unknown,
                status: Status::Unstable,
                is_git: true,
                last_commit: "N/A".to_owned(),
            })
        })
        .expect("AAAA")
        .collect();

    let mut write_stmt = conn.prepare(WRITE_TMP_TO_PROJECT)?;
    for x in tmp_project? {
        write_stmt
            .execute([x.path, x.name, x.category.to_string(), x.status.to_string()])
            .expect("AAA!");
    }

    Ok(())
}

///
fn regex_repo(path: String) -> String {
    let re = Regex::new(r"(.+)/([^/]+)").expect("AAAA");
    let caps = re.captures(&path).unwrap();

    caps.get(0).unwrap().as_str().to_string()
}