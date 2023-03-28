use rusqlite::Connection;

pub mod project;
pub mod tmp_config;
pub mod todo;
pub mod category;

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
const CREATE_TODO: &str = "CREATE TABLE IF NOT EXISTS todo (
    id          integer primary key autoincrement,
    parent_id   integer,
    project_id  integer,
    todo        varchar(255),
    is_complete tinyint(1) default 0,
    priority    integer default 1
);";
const CREATE_CATEGORIES: &str = "CREATE TABLE IF NOT EXISTS category (
    id   integer primary key autoincrement,
    name varchar(255)
);";
const CREATE_CONFIG: &str = "CREATE TABLE IF NOT EXISTS tmp_git_config (
    path        varchar(255) not null primary key, 
    is_selected tinyint(1) default 0, 
    UNIQUE(path)
);";

pub fn initialize_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(CREATE_PROJECT, ())?;
    conn.execute(CREATE_TODO, ()).expect("CREATE_TODO_ERROR");
    conn.execute(CREATE_CATEGORIES, ())?;
    conn.execute(CREATE_CONFIG, ())?;

    Ok(())
}
