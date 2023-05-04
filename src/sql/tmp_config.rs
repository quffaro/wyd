use crate::app::regex::regex_last_dir;
use crate::app::structs::gitconfig::{fetch_config_files, guess_git_owner, GitConfig};
use crate::app::structs::projects::{Project, ProjectBuilder, ProjectStatus};
use crate::{home_path, PATH_DB};
use rusqlite::{Connection, Result};

const CREATE_CONFIG: &str = "CREATE TABLE IF NOT EXISTS tmp_git_config (
    path        varchar(255) not null primary key, 
    is_selected tinyint(1) default 0, 
    UNIQUE(path)
);";
///
const READ_TMP: &str = "select path, is_selected from tmp_git_config where is_selected = 0 and path not in (select path from project)";
pub fn read_tmp(conn: &Connection) -> Result<Vec<GitConfig>, rusqlite::Error> {
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
const UPDATE_TMP: &str = "update tmp_git_config set is_selected = ?1 where path = ?2;";
pub fn update_tmp(conn: &Connection, tmp: &GitConfig) -> Result<(), rusqlite::Error> {
    conn.execute(UPDATE_TMP, (&tmp.is_selected, &tmp.path))
        .expect("A");

    Ok(())
}
/// WRITE TEMPORARY PROJECTS
const INSERT_INTO_TMP: &str = "INSERT OR IGNORE INTO tmp_git_config (path) VALUES (?)";
pub fn write_tmp(conn: &Connection, tmp: Vec<String>) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(INSERT_INTO_TMP)?;
    for x in tmp {
        stmt.execute([x])?;
    }

    Ok(())
}

pub fn init_tmp_git_config() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(home_path(PATH_DB)).unwrap();
    let tmp = fetch_config_files();
    write_tmp(&conn, tmp);

    Ok(())
}

/// all new tmps are written to
// const READ_TMP_SELECTED: &str    = "select path from tmp_git_config where is_selected = 1";
const READ_TMP_SELECTED: &str = "select path from tmp_git_config where is_selected = 1 and path not in (select path from project)";
const WRITE_TMP_TO_PROJECT: &str =
    "insert or replace into project (path,name,cat,status,is_git,owner,repo) values (?1, ?2, ?3, ?4,?5,?6,?7);";
pub fn write_tmp_to_project(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut read_stmt = conn.prepare(READ_TMP_SELECTED)?;

    let tmp_project: Result<Vec<Project>, rusqlite::Error> = read_stmt
        .query_map([], |row| {
            // TODO error handle git2 error for rusqlite
            let project: Project = ProjectBuilder::new()
                .path(row.get(0)?)
                .name(regex_last_dir(row.get(0)?))
                .desc("Example".to_owned())
                .category("Unknown".to_owned())
                .status(ProjectStatus::Unstable)
                .is_git(true)
                .owner(guess_git_owner(row.get(0)?).unwrap())
                .repo(regex_last_dir(row.get(0)?))
                .last_commit("N/A".to_owned())
                .build();
            Ok(
                project, // Project {
                        // id: 0,
                        // path: row.get(0)?,
                        // name: regex_last_dir(row.get(0)?),
                        // desc: "Example".to_owned(),
                        // category: "Unknown".to_owned(),
                        // status: ProjectStatus::Unstable,
                        // is_git: true,
                        // owner: guess_git_owner(row.get(0)?),
                        // repo: regex_last_dir(row.get(0)?),
                        // last_commit: "N/A".to_owned(),
                        // }
            )
        })
        .expect("AAAA")
        .collect();

    let mut write_stmt = conn.prepare(WRITE_TMP_TO_PROJECT)?;
    for x in tmp_project? {
        write_stmt
            .execute([
                x.path,
                x.name,
                x.category.to_string(),
                x.status.to_string(),
                1.to_string(),
                x.owner,
                x.repo,
            ])
            .expect("AAA!");
    }

    Ok(())
}
