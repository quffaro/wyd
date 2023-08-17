use crate::app::structs::category::Category;
use serde::{Deserialize, Serialize};

// pub fn read_category() -> Result<Vec<Category>> {
// let mut

// const READ_CATEGORY: &str = "select id,name from category";
// pub fn read_category(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
//     let mut stmt = conn.prepare(READ_CATEGORY)?;
//     let res = stmt
//         .query_map([], |row| {
//             Ok(Category {
//                 id: row.get(0)?,
//                 name: row.get(1)?,
//             })
//         })
//         .expect("A!!")
//         .collect();

//     res
// }

// const INSERT_INTO_CATEGORY: &str = "INSERT OR IGNORE INTO category (name) VALUES (?)";
// pub fn write_category(conn: &Connection, cat: &String) -> Result<(), rusqlite::Error> {
//     let mut stmt = conn.prepare(INSERT_INTO_CATEGORY)?;
//     stmt.execute([cat])?;

//     Ok(())
// }
