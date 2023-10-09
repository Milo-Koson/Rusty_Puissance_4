use clearscreen;

mod chronometre;
mod players;
mod game_data;

fn main() {
    /* Launch thread objects */
    // Creation of StateManager

    let player1_name = players::get_player_name(1);
    println!();
    let player2_name = players::get_player_name(2);
    println!();

    // Créez les joueurs avec les noms saisis
    let player1 = players::Player::new(&player1_name, players::IdPlayer::Player1, "X");
    let player2 = players::Player::new(&player2_name, players::IdPlayer::Player2, "O");

    let mut current_game = game_data::GameData::new(player1_name, player2_name);

    while !current_game.game_over {
        // Obtenez le joueur actuel
        let current_player = &current_game.players[current_game.current_player];
    
        // Affichez la grille actuelle (avant que le jeu ne commence)
        current_game.display();
    
        println!("C'est à {} de jouer ({}).", current_player.name, current_player.symbol);
    
        let mut valid_move = false;
        while !valid_move {
            // Demandez au joueur de choisir une colonne
            let column = players::get_column_choice();
    
            // Essayez de faire le coup
            match current_game.make_move(column) {
                Ok(_) => {
                    valid_move = true;
                }
                Err(err) => {
                    println!("Erreur : {}", err);
                }
            }
            
            // Effacement de la grille de jeu pour actualiser le terminal
            clearscreen::clear().expect("Échec de l'effacement de l'écran !");
        }
        // Vérifiez s'il y a une victoire ou un match nul ici
    }

    println!("Finish main program")
}

