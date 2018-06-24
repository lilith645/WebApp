use std::sync::Mutex;

use rusqlite::{Connection, Error};

pub type DbConn = Mutex<Connection>;

pub fn open_db_in_memory() -> Connection {
  Connection::open_in_memory().expect("in memory db")
}

pub fn init_database(conn: &Connection) {
  conn.execute("CREATE TABLE entries (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL
                )", &[])
      .expect("create entries table");

  conn.execute("INSERT INTO entries (id, name) VALUES ($1, $2)",
          &[&0, &"Rocketeer"])
      .expect("insert single entry into entries table");
}
