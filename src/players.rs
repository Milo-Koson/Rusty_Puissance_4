use std::io;

pub enum IdPlayer {
    Player1,
    Player2
}

pub struct Player {
    pub name: String,
    pub id: IdPlayer,
    pub symbol: String
}

impl Player {
    pub fn new(name: &str, id: IdPlayer, symbol: &str) -> Self {
        Player {
            name: name.to_string(),
            id,
            symbol: symbol.to_string()
        }
    }
}

pub fn get_player_name(player_number: u8) -> String {
    println!("Entrez le nom du joueur {} :\n", player_number);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("La saisie contient une erreur, veuillez recommencer !");

    let trimmed_name = input.trim();
    trimmed_name.to_string()
}