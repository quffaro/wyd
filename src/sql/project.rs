use crate::app::structs::projects::{Project, ProjectBuilder};
use rusqlite::{params, Connection, Result};
use std::env;

const READ_PROJECT: &str =
    "select id,path,name,desc,cat,status,is_git,owner,repo,last_commit,sort from project";
pub fn read_project(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
    let mut stmt = conn.prepare(READ_PROJECT)?;
    let res = stmt
        .query_map([], |row| {
            let project: Project = ProjectBuilder::new()
                .id(row.get(0)?)
                .path(row.get(1)?)
                .name(row.get(2)?)
                .desc(match row.get(3)? {
                    Some(x) => x,
                    None => "N/A".to_owned(),
                })
                .category(row.get(4)?)
                .status(row.get(5)?)
                .is_git(row.get(6)?)
                .owner(row.get(7)?)
                .repo(row.get(8)?)
                .last_commit(row.get(9)?)
                .sort(row.get(10)?)
                .build();
            Ok(project)
        })
        .expect("A!!")
        .collect();

    res
}

const READ_V_PROJECT: &str =
    "select id,path,name,desc,cat,status,is_git,owner,repo,last_commit,sort from v_project";
pub fn read_v_project(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
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
                sort: row.get(10)?,
            })
        })
        .expect("A!!")
        .collect();

    res
}

/// READ PROJECTS FROM DB
const READ_PROJECT_REPOS: &str =
    "select id,path,name,desc,cat,status,is_git,owner,repo,last_commit,sort from project where repo is not null and owner is not null";
pub fn read_project_repos(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
    let mut stmt = conn.prepare(READ_PROJECT_REPOS)?;
    let res = stmt
        .query_map([], |row| {
            let project: Project = ProjectBuilder::new()
                .id(row.get(0)?)
                .path(row.get(1)?)
                .name(row.get(2)?)
                .desc(match row.get(3)? {
                    Some(x) => x,
                    None => "N/A".to_owned(),
                })
                .category(row.get(4)?)
                .status(row.get(5)?)
                .is_git(row.get(6)?)
                .owner(row.get(7)?)
                .repo(row.get(8)?)
                .last_commit(row.get(9)?)
                .sort(row.get(10)?)
                .build();
            Ok(project)
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
    last_commit,
    sort
) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);";
const INSERT_INTO_PROJECT_PATH: &str = "insert into project_path (
    path,
    project_id
) values (?1, ?2);";
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
        project.sort,
    ]);
    let mut path_stmt = conn.prepare(INSERT_INTO_PROJECT_PATH)?;
    path_stmt.execute(params![project.path, conn.last_insert_rowid(),]);

    Ok(())
}

const UPDATE_PROJECT_DESC: &str = "update project set desc = ?1 where id = ?2;";
pub fn update_project_desc(
    conn: &Connection,
    pid: u8,
    desc: String,
) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_PROJECT_DESC, (desc, pid)).expect("A");

    Ok(())
}
const UPDATE_PROJECT_CAT: &str = "update project set cat = ?1 where id = ?2;";
pub fn update_project_category(
    conn: &Connection,
    project: &Project,
    category: &String,
) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_PROJECT_CAT, (format!("{}", &category), project.id))
        .expect("A");

    Ok(())
}

const UPDATE_PROJECT_LAST_COMMIT: &str = "update project set last_commit = ?1 where id = ?2;";
pub fn update_project_last_commit(
    conn: &Connection,
    project: &Project,
    last_commit: String,
) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_PROJECT_LAST_COMMIT, params![last_commit, project.id])
        .expect("AAA");

    Ok(())
}

const UPDATE_PROJECT_SORT: &str = "update project set sort = ?1 where id = ?2;";
pub fn update_project_sort(
    conn: &Connection,
    p_id: usize,
    sort: usize,
) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_PROJECT_SORT, params![sort, p_id])
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

const DELETE_PROJECT: &str = "delete from project where id = ?1; 
    delete from project_path where project_id = ?1;
    delete from todo where project_id = ?1";
pub fn delete_project(conn: &Connection, project: &Project) -> Result<(), rusqlite::Error> {
    conn.execute(DELETE_PROJECT, [project.id]).expect("AAA");

    Ok(())
}

pub fn add_project_in_dir(is_find_git: bool, conn: &Connection) {
    let path = env::current_dir().unwrap().display().to_string();
    let copy = path.clone();
    let project_build = ProjectBuilder::new()
        .path(path)
        .desc("N/A".to_owned())
        .category("Unknown".to_owned())
        .status(crate::app::structs::projects::ProjectStatus::Unstable)
        .is_git(true)
        .owner("Unknown".to_owned()) // TODO get from config
        .repo("".to_owned()) // TODO default)
        .last_commit("".to_owned());
    if is_find_git {
        match git2::Repository::discover(copy) {
            Ok(r) => {
                let repo = r.workdir().unwrap().to_str().unwrap().to_string();
                write_project(
                    conn,
                    project_build
                        .owner(
                            crate::app::structs::gitconfig::guess_git_owner(repo.clone()).unwrap(),
                        )
                        .name(crate::app::regex::regex_last_dir(repo.clone()))
                        .repo(crate::app::regex::regex_last_dir(repo.clone()))
                        .build(),
                );
            }
            Err(_) => (), // TODO log that nothing happened here
        }
    } else {
        write_project(conn, project_build.build());
    }
}
