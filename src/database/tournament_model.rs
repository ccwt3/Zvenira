#[derive(Debug)]
pub enum PairingSystems {
    Swiss,
    RoundRobin,
    Teams,
    Knockout,
    Null,
}

#[derive(Debug)]
pub struct Tournament {
    name: String,
    pairing_system: PairingSystems,
    total_players: u32,
    total_rounds: u32,
}

impl Tournament {
    pub fn new(name: String, pairing_system: PairingSystems, total_players: u32) -> Self {
        let total_rounds = 0;

        Self {
            name,
            pairing_system,
            total_players,
            total_rounds,
        }
    }

    // GETTERS

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pairing_system(&self) -> &PairingSystems {
        &self.pairing_system
    }

    pub fn total_players(&self) -> &u32 {
        &self.total_players
    }

    pub fn total_rounds(&self) -> &u32 {
        &self.total_rounds
    }

    // SETTERS
    // You cannot change the pairing system, the total rounds and (for now) the players amount
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    // METHODS

    pub fn print_tournament(&self) {
        println!("{}", self.name);
    }
}
