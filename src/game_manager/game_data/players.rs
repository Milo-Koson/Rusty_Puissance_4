use std::io;

/**
Énumération qui comprend les 2 joueurs du jeu
*/
pub enum IdPlayer {
    Player1,
    Player2
}

/**
Structure qui comprend le nom, l'id et le symbole attribués à chacun des joueurs
*/
pub struct Player {
    pub name: String,
    pub id: IdPlayer,
    pub symbol: String
}

impl Player {

    /**
    Création d'une instance de la structure Player
    */
    pub fn new(name: &str, id: IdPlayer, symbol: &str) -> Self {
        Player {
            name: name.to_string(),
            id,
            symbol: symbol.to_string()
        }
    }

}

/**
Fonction qui demande au joueur de saisir son nom (la saisie ne peut pas être vide)
*/
pub fn input_player_name(player_number: u8) -> String {
    loop {
        println!("Entrez le nom du joueur {} :\n", player_number);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("La saisie contient une erreur, veuillez recommencer !");

        let trimmed_name = input.trim();
        if !trimmed_name.is_empty() {
            return trimmed_name.to_string();
        } else {
            println!("Le nom ne peut pas être vide. Veuillez entrer un nom valide.");
        }
    }
}

/**
Fonction qui va définir le nom des joueurs avec les entrées de la fonction input_player_names
(la saisie ne peut être qu'un chiffre entre 1 et 7 et ne peut pas être vide)
*/
pub fn set_player_names() -> (String, String) {
    let player1_name = input_player_name(1);
    println!();
    let player2_name = input_player_name(2);
    println!();

    (player1_name, player2_name)
}

/**
Fonction qui va demander à l'utilisateur de choisir la colonne de la grille de jeu
dans laquelle il souhaite placer son jeton
*/
pub fn get_column_choice() -> usize {
    loop {
        println!("Entrez le numéro de la colonne où vous souhaitez placer votre pièce : ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("La saisie contient une erreur, veuillez recommencer");

        match input.trim().parse::<usize>() {
            Ok(column) if (1..=7).contains(&column) => return column - 1,
            _ => {
                println!("La réponse n'est pas valide. Veuillez entrer un chiffre entre 1 et 7.");
                continue;
            }
        }
    }
}