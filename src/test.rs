use clearscreen;

mod chronometre;
mod players;
mod game_data;
fn main() {
    let player2_name = players::get_player_name(2);
    println!();

    // Créez les joueurs avec les noms saisis
    let player1 = players::Player::new(&player1_name, players::IdPlayer::Player1, "X");
    let player2 = players::Player::new(&player2_name, players::IdPlayer::Player2, "O");

    // Modifie les noms des joueurs avec les noms saisis
    let mut current_game = game_data::GameData::new(player1_name, player2_name);

    while !current_game.game_over {
        // Obtenez le joueur actuel
        // Détermine le joueur courant
        let current_player = &current_game.players[current_game.current_player];

        // Affichez la grille actuelle (avant que le jeu ne commence)
        // Affiche la grille vide
        current_game.display();

        println!("C'est à {} de jouer ({}).", current_player.name, current_player.symbol);

        let mut valid_move = false;
        while !valid_move {
            // Demandez au joueur de choisir une colonne
            // Demande au joueur courant de choisir la colonne
            let column = players::get_column_choice();

            // Essayez de faire le coup
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
        // Vérifiez s'il y a une victoire ou un match nul ici
        // Vérifie s'il y a un match nul ou une victoire
        current_game.is_game_draw();
        current_game.is_game_over();

        // Actualise le joueur l'état de jeu et le joueur courant en cas de victoire
        if current_game.is_game_over() {
            current_game.game_over = true;
            current_game.current_player = 1 - current_game.current_player;
        }
    }

    println!("Finish main program");
    // Affiche le gagnant
    current_game.display();
    println!("Le gagnant est : {} ", current_game.get_name());
    //TODO encapsuler la fonction pour avoir un .get_name();
    println!("Fin du jeu !")
}