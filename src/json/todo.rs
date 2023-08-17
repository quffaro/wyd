use crate::app::structs::todos::Todo;

/// TODOs
pub fn read_todo(conn: &Connection) -> Result<Vec<Todo>, rusqlite::Error> {
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

pub fn write_new_todo(conn: &Connection, todos: Vec<Todo>) -> Result<(), rusqlite::Error> {
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

pub fn update_todo(conn: &Connection, todo: &Todo) -> Result<(), rusqlite::Error> {
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

pub fn update_todo_priority(id: u8, priority: u8) -> Result<(), rusqlite::Error> {
    write_stmt.execute(params![priority, id]).expect("AAAA");

    Ok(())
}
