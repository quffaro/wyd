use rusqlite::Connection;

pub mod category;
pub mod project;
pub mod tmp_config;
pub mod todo;

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
const CREATE_PROJECT_PATH: &str = "CREATE TABLE IF NOT EXISTS project_path (
    id          integer primary key autoincrement,
    project_id  integer,
    author      varchar(255),
    path        varchar(4000)
);";
const CREATE_VIEW_PROJECT: &str = "
    DROP VIEW IF EXISTS v_project;
    CREATE VIEW v_project
    AS SELECT
    `t`.`id`          AS `id`,
    `s`.`path`        AS `path`,
    `t`.`name`        AS `name`,
    `t`.`cat`         AS `cat`,
    `t`.`status`      AS `status`,
    `t`.`is_git`      AS `is_git`,
    `t`.`owner`       AS `owner`,
    `t`.`repo`        AS `repo`,
    `t`.`last_commit` AS `last_commit`
    FROM project t
    LEFT JOIN project_path s
    ON `t`.`id` = `s`.`project_id`;";
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
    conn.execute(CREATE_PROJECT_PATH, ())?;
    conn.execute(CREATE_VIEW_PROJECT, ())?;
    conn.execute(CREATE_TODO, ()).expect("CREATE_TODO_ERROR");
    conn.execute(CREATE_CATEGORIES, ())?;
    conn.execute(CREATE_CONFIG, ())?;

    Ok(())
}
