mod state_manager;
mod chronometre;
mod displayer;
mod grid;
mod players;
mod game_data;

use std::thread;
use std::time::Duration;

use crate::game_data::GameData;

fn main() {
    /* Launch thread objects */
    // Creation of StateManager

    let player1_name = players::get_player_name(1);
    println!();
    let player2_name = players::get_player_name(2);
    println!();

    // Créez les joueurs avec les noms saisis
    let player1 = players::Player::new(&player1_name, players::IdPlayer::Player1, "O");
    let player2 = players::Player::new(&player2_name, players::IdPlayer::Player2, "X");

    let mut game = game_data::GameData::new(player1_name, player2_name);

    GameData::display(&game);
    
    println!("Finish main program")
}

