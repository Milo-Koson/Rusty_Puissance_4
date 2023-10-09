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

    // Modifie les noms des joueurs avec les noms saisis
    let mut current_game = game_data::GameData::new(player1_name, player2_name);

    while !current_game.game_over {
        // Détermine le joueur courant
        let current_player = &current_game.players[current_game.current_player];
    
        // Affiche la grille vide
        current_game.display();
    
        println!("C'est à {} de jouer ({}).", current_player.name, current_player.symbol);
    
        let mut valid_move = false;
        while !valid_move {
            // Demande au joueur courant de choisir la colonne
            let column = players::get_column_choice();
    
            // Essayez de placer une pièce sur la grille
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
        // Vérifie s'il y a un match nul ou une victoire
        current_game.is_game_draw();
        current_game.is_game_over();

        // Actualise le joueur l'état de jeu et le joueur courant en cas de victoire
        if current_game.is_game_over() {
            current_game.game_over = true;
            current_game.current_player = 1 - current_game.current_player;
        }
    }

    // Affiche le gagnant
    current_game.display();
    println!("Le gagnant est : {} ", current_game.get_name());
    //TODO encapsuler la fonction pour avoir un .get_name();
    println!("Fin du jeu !")
}

