use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Player {
    name: String,
    country: String,
    federation: String,
    elo: u32,
}

//DEVELOPER NOTE
// Add database logic within the player class so it can be a good use of models naming

impl Player {
    pub fn new(name: String, country: String, federation: String, elo: u32) -> Self {
        Self {
            name,
            country,
            federation,
            elo,
        }
    }

    // GETTERS
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn federation(&self) -> &str {
        &self.federation
    }

    pub fn elo(&self) -> &u32 {
        &self.elo
    }

    // SETTERS
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_country(&mut self, new_country: String) {
        self.country = new_country;
    }

    pub fn deration(&mut self, new_federation: String) {
        self.federation = new_federation;
    }

    pub fn set_elo(&mut self, new_elo: u32) {
        self.elo = new_elo;
    }

    // METHODS

    pub fn save(&self) -> Result<()> {
        println!("{}", self.name);

        let conn = Connection::open("data/zvenira.db")?;

        conn.execute(
            "INSERT INTO Player (name, country, federation, elo) VALUES (?1, ?2, ?3, ?4)",
            params![&self.name, &self.country, &self.country, &self.elo],
        )?;

        Ok(())
    }
}
