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

pub fn set_player_names() -> (String, String) {
    let player1_name = get_player_name(1);
    println!();
    let player2_name = get_player_name(2);
    println!();

    (player1_name, player2_name)
}

pub fn get_column_choice() -> usize {
    println!("Entrez le numéro de la colonne où vous souhaitez placer votre pièce : ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("La saisie contient une erreur, veuillez recommencer");

    let column: usize = input.trim().parse().expect("La colonne n'est pas valide, veuillez en choisir une autre");

    column - 1
}