use crate::app::structs::todos::Todo;
use rusqlite::{params, Connection, Result};

/// TODOs
const CREATE_TODO: &str = "CREATE TABLE IF NOT EXISTS todo (
    id          integer primary key autoincrement,
    parent_id   integer,
    project_id  integer,
    todo        varchar(255),
    is_complete tinyint(1) default 0,
    priority    integer default 1
);";
const READ_TODO: &str = "select id,parent_id,project_id,todo,is_complete,priority from todo";
pub fn read_todo(conn: &Connection) -> Result<Vec<Todo>, rusqlite::Error> {
    let mut stmt = conn.prepare(READ_TODO)?;
    let res = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                project_id: row.get(2)?,
                todo: row.get(3)?,
                is_complete: row.get(4)?,
                priority: row.get(5)?,
            })
        })
        .expect("A!!")
        .collect();

    res
}
const WRITE_NEW_TODO: &str = "insert or replace into todo (
        parent_id,
        project_id,
        todo,
        is_complete
) values (?1, ?2, ?3, ?4);";
pub fn write_new_todo(conn: &Connection, todos: Vec<Todo>) -> Result<(), rusqlite::Error> {
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
const UPDATE_TODO: &str = "insert or replace into todo (
    id,
    parent_id,
    project_id,
    todo,
    is_complete,
    priority
) values (?1, ?2, ?3, ?4, ?5, ?6);";
pub fn update_todo(conn: &Connection, todo: &Todo) -> Result<(), rusqlite::Error> {
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
            todo.priority,
        ])
        .expect("AAA!");

    Ok(())
}
const UPDATE_TODO_PRIORITY: &str = "update todo set priority = ?1 where id = ?2;";
pub fn update_todo_priority(
    conn: &Connection,
    id: u8,
    priority: u8,
) -> Result<(), rusqlite::Error> {
    let mut write_stmt = conn.prepare(UPDATE_TODO_PRIORITY)?;
    write_stmt.execute(params![priority, id]).expect("AAAA");

    Ok(())
}
