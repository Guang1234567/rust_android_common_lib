use std::path::Path;

use rusqlite::{Connection, params, Result, NO_PARAMS};
use rusqlite::types::ToSql;

use time::Timespec;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

pub struct SqliteHelper {}


impl SqliteHelper {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Connection> {
        Connection::open(path)
    }

    pub fn open_in_memory() -> Result<Connection> {
        Connection::open_in_memory()
    }

    pub fn write_sth_to_db(mut conn: Connection) -> Result<()> {
        android_log::init("app_rust_sql").unwrap();

        let tx = conn.transaction()?;

        tx.execute(
            "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
            NO_PARAMS,
        )?;
        let me = Person {
            id: 0,
            name: "Steven".to_string(),
            time_created: time::get_time(),
            data: None,
        };
        tx.execute(
            "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
            params![me.name, me.time_created, me.data],
        )?;

        tx.commit();

        let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person")?;
        let person_iter = stmt.query_map(params![], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                time_created: row.get(2)?,
                data: row.get(3)?,
            })
        })?;

        for person in person_iter {
            warn!("Found person {:?}", person?);
        }
        Ok(())
    }
}