use rusqlite::{Connection, Error};
use std::path::Path;
use serde;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Config {
    pub name: String,
    pub email: String
}

static DB: &str = "./app.db";

fn db_path() -> &'static Path {
	Path::new(DB)
}

fn create() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path())?;

    let result = conn.execute(
        "CREATE TABLE config (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT
        )",
        ()
    );

	match result {
		Ok(_) => Ok(()),
		Err(err) => Err(err)
	}
}

pub fn init() -> Result<(), rusqlite::Error> {
    if db_path().exists() {
        create()
    } else {
        Ok(())
    }
}

pub fn read() -> Result<Config, Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare("SELECT id, name, email FROM config")?;
    let config_iter = stmt.query_map([], |row| {
        Ok(Config {
            name: row.get(1)?,
            email: row.get(2)?
        })
    })?;

	let mut configs = Vec::new();
    for configs_result in config_iter {
        configs.push(configs_result?);
    }

    match configs.first() {
		Some(config) => Ok(config.clone()),
		_ => Err(Error::QueryReturnedNoRows)
	}
}

pub fn insert(config: Config) -> Result<usize, rusqlite::Error> {
    let conn = Connection::open(DB)?;

    conn.execute(
        "INSERT INTO config (id, name, email) VALUES (?1, ?2, ?3) \
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, email=excluded.email \
             WHERE true;",
        (0, &config.name, &config.email),
    )
}

