use crate::app::structs::projects::Project;
use rusqlite::{params, Connection, Result};

const CREATE_PROJECT: &str = "CREATE TABLE IF NOT EXISTS project (
    id          integer primary key autoincrement, 
    path        varchar(255), 
    name        varchar(255), 
    desc        varchar(4000), 
    cat         varchar(255), 
    status      varchar(255),
    is_git      tinyint(1),
    owner       varchar(255),
    repo        varchar(255),
    last_commit varchar(255),
    UNIQUE(path)
);";
const READ_PROJECT: &str =
    "select id,path,name,desc,cat,status,is_git,owner,repo,last_commit from project";
pub fn read_project(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
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
                owner: row.get(7)?,
                repo: row.get(8)?,
                last_commit: match row.get(9)? {
                    Some(x) => x,
                    None => "N/A".to_string(),
                },
            })
        })
        .expect("A!!")
        .collect();

    res
}
/// READ PROJECTS FROM DB
const READ_PROJECT_REPOS: &str =
    "select id,path,name,desc,cat,status,is_git,owner,repo,last_commit from project where repo is not null and owner is not null";
pub fn read_project_repos(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
    let mut stmt = conn.prepare(READ_PROJECT_REPOS)?;
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
                owner: row.get(7)?,
                repo: row.get(8)?,
                last_commit: match row.get(9)? {
                    Some(x) => x,
                    None => "N/A".to_string(),
                },
            })
        })
        .expect("A!!")
        .collect();

    res
}
/// WRITE PROJECT TO DB
const INSERT_PROJECT: &str = "insert into project (
    path,
    name, 
    desc, 
    cat, 
    status, 
    is_git,
    owner,
    repo,
    last_commit
) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";
pub fn write_project(conn: &Connection, project: Project) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(INSERT_PROJECT)?;
    stmt.execute(params![
        project.path,
        project.name,
        project.desc,
        project.category.to_string(),
        project.status.to_string(),
        project.is_git,
        project.owner,
        project.repo,
        project.last_commit,
    ]);

    Ok(())
}
const UPDATE_PROJECT_DESC: &str = "update project set desc = ?1 where id = ?2;";
pub fn update_project_desc(
    conn: &Connection,
    project: &Project,
    desc: String,
) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_PROJECT_DESC, (desc, &project.id))
        .expect("A");

    Ok(())
}
const UPDATE_PROJECT_CAT: &str = "update project set cat = ?1 where id = ?2;";
pub fn update_project_category(
    conn: &Connection,
    project: &Project,
    cat: &String,
) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_PROJECT_CAT, (format!("{}", &cat), project.id))
        .expect("A");

    Ok(())
}

const UPDATE_PROJECT_LAST_COMMIT: &str = "update project set last_commit = ?1 where id = ?2;";
pub fn update_project_last_commit(
    conn: &Connection,
    project: &Project,
    last_commit: String,
) -> Result<(), rusqlite::Error> {
    // println!("{:#?},{:#?}", project, &last_commit);
    conn.execute(UPDATE_PROJECT_LAST_COMMIT, params![last_commit, project.id])
        .expect("AAA");

    Ok(())
}

const UPDATE_PROJECT_STATUS: &str = "update project set status = ?1 where id = ?2;";
pub fn update_project_status(conn: &Connection, project: &Project) -> Result<(), rusqlite::Error> {
    conn.execute(
        UPDATE_PROJECT_STATUS,
        params![format!("{}", project.status), project.id],
    )
    .expect("AAA");

    Ok(())
}
