use zvenira::database;

fn main() {
    let tournament1 =
        database::Tournament::new("World Cup".to_string(), database::PairingSystems::Swiss, 10);

    tournament1.print_tournament();
}
