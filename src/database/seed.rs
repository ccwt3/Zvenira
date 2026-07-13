use rusqlite::{Connection, Result, params};
use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

pub fn populate() -> Result<()> {
    let conn = Connection::open("data/cats.db")?;

    let mut cat_colors: HashMap<String, Vec<&str>> = HashMap::new();

    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute("INSERT INTO cat_colors (name) VALUES (?1)", [color])?;

        let last_id = conn.last_insert_rowid();

        for cat in catnames {
            conn.execute(
                "
      INSERT INTO cats (name, color_id) values (?1, ?2)
      ",
                params![cat, last_id],
            )?;
        }
    }

    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name FROM cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        if let Ok(found_cat) = cat {
            println!(
                "Found cat {:?} {} is {}",
                found_cat, found_cat.name, found_cat.color,
            );
        }
    }

    Ok(())
}
