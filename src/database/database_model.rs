use rusqlite::{Connection, Result};

// Database V1, this version does not allow participant withdrawal mid tournament

pub fn create_database() -> Result<()> {
    let conn = Connection::open("data/zvenira.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Player (
           id INETEGER PRIMARY KEY,
           name TEXT NOT NULL UNIQUE,
           country TEXT NOT NULL,
           federation text NOT NULL,
           elo INTEGER NOT NULL
       )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Tournament (
           id INTEGER PRIMARY KEY,
           name TEXT NOT NULL,
           pairing_system TEXT NOT NULL,
           total_players INTEGER NOT NULL,
           total_rounds INTEGER NOT NULL
       )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_round (
           id integer primary key,
           player_id INTEGER REFERENCES Player(id),
           oponent_id INTEGER REFERENCES Player(id),
           tournament_id INTEGER REFERENCES Tournament(id),
           round INTEGER NOT NULL,
           color TEXT NOT NULL,
           current_score REAL NOT NULL,
           result TEXT NOT NULL
       )",
        (),
    )?;

    Ok(())
}
