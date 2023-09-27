use crate::grid::{create_grid, display_grid};
use crate::players::{Player, IdPlayer, get_player_name}; 

pub struct GameData {
    pub grid: Vec<Vec<char>>,
    pub players: [Player; 2],
    pub current_player: usize,
    pub game_over: bool
}

impl GameData {
    pub fn new(player1_name: String, player2_name: String) -> GameData {
        let grid = create_grid(6, 7);
        let player1 = Player::new(&player1_name, IdPlayer::Player1, "O");
        let player2 = Player::new(&player2_name, IdPlayer::Player2, "X");
        
        GameData {
            grid,
            players: [player1, player2],
            current_player: 0,
            game_over: false
        }
    }
    
    pub fn display(&self) {
        display_grid(&self.grid);
    }
}

