use rusqlite::{Connection, Result};

pub fn create_database() -> Result<()> {
    let conn = Connection::open("data/cats.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cat_colors (
           id integer primary key,
           name text not null unique
       )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cats (
           id integer primary key,
           name text not null,
           color_id integer not null references cat_colors(id)
       )",
        (),
    )?;

    Ok(())
}
