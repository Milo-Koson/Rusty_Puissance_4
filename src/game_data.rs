mod grid;
use crate::players::{Player, IdPlayer}; 

pub struct GameData {
    pub grid: Vec<Vec<char>>,
    pub players: [Player; 2],
    pub current_player: usize,
    pub game_over: bool
}

impl GameData {
    pub fn new(player1_name: String, player2_name: String) -> GameData {
        let grid = grid::create_grid(6, 7);
        let player1 = Player::new(&player1_name, IdPlayer::Player1, "X");
        let player2 = Player::new(&player2_name, IdPlayer::Player2, "O");
        
        GameData {
            grid,
            players: [player1, player2],
            current_player: 0,
            game_over: false
        }
    }

    pub fn display(&self) {
        grid::display_grid(&self.grid);
    }

    pub fn make_move(&mut self, column: usize) -> Result<(), &str> {
       
        if column >= self.grid[0].len() {
            return Err("La colonne n'est pas valide, veuillez en choisir une autre");
        }

        let mut row = None;
        for r in (0..self.grid.len()).rev() {
            if self.grid[r][column] == ' ' {
                row = Some(r);
                break;
            }
        }

        if row.is_none() {
            return Err("La colonne est pleine, veuillez en choisir une autre");
        }

        // Placez le jeton du joueur actuel dans la grille
        let current_player = &self.players[self.current_player];
        self.grid[row.unwrap()][column] = current_player.symbol.chars().next().unwrap();

        // Vérifiez s'il y a une victoire ou un match nul (implémentez cette logique ici)

        // Passez au joueur suivant
        self.current_player = 1 - self.current_player;

        Ok(())
    }

}

