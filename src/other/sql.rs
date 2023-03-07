use crate::other::viewer::{GitConfig, Project, Status};
use rusqlite::Connection;
use wyd::{self, DATABASE};

//
pub fn read_tmp() -> Result<Vec<GitConfig>, rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

    let mut stmt =
        conn.prepare("select path, is_selected from tmp_git_config where is_selected = 0")?;
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

pub fn read_project() -> Result<Vec<Project>, rusqlite::Error> {
    let conn = Connection::open(wyd::DATABASE)?;

    let mut stmt = conn.prepare("select id,path,name,cat,status,last_commit from project")?;
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

pub fn update_tmp(tmp: &GitConfig) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;

    conn.execute(
        "update tmp_git_config set is_selected = ?1 where path = ?2;",
        (&tmp.is_selected, &tmp.path),
    )
    .expect("A");

    Ok(())
}

// all new tmps are written to
pub fn write_tmp_to_project() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE)?;
    // match Regex::new(r#"/^(.+)\/([^\/]+)$/gm"#) {
    //     Ok(r) => {
    //         let re = r;
    //     }
    //     Err(e) => {
    //         let path = "error.txt";
    //         let mut output = File::create(path).unwrap();
    //         write!(output, "{}", format!("{:?}", e))
    //     }
    // }

    let mut stmt = conn.prepare(
        "insert or replace into project (path,name,cat,status) values (?1, ?2, ?3, ?4);",
    )?;
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
