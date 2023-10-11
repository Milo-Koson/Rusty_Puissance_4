mod grid;

use crate::players::{Player, IdPlayer, self}; 
use crate::players::get_column_choice;

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

    pub fn get_name(&self) -> &str {
        &self.players[self.current_player].name
    }

    pub fn display(&self) {
        grid::display_grid(&self.grid);
    }

    pub fn make_move(&mut self, column: usize) -> Result<(), &str> {
       
        if column >= self.grid[0].len() {
            return Err("La colonne n'est pas valide, veuillez en choisir une autre");
        }

        // Ancienne version de la boucle
        let mut row = None;
        for r in (0..self.grid.len()).rev() {
            if self.grid[r][column] == ' ' {
                row = Some(r);
                break;
            }
        }

        // let row = self.grid.iter().find(|r| r[column] == ' ');

        if row.is_none() {
            return Err("La colonne est pleine, veuillez en choisir une autre");
        }

        // Placez le jeton du joueur actuel dans la grille
        let current_player = &self.players[self.current_player];
        self.grid[row.unwrap()][column] = current_player.symbol.chars().next().unwrap();

        // Actualise le joueur courant
        self.current_player = 1 - self.current_player;

        Ok(())
    }

    pub fn play_game(&mut self) {

        // Détermine le joueur courant
        let current_player = &self.players[self.current_player];
        
        // Affiche la grille vide
        self.display();
    
        println!("C'est à {} de jouer ({}).", current_player.name, current_player.symbol);
    
        let mut valid_move = false;
        while !valid_move {
            // Demande au joueur courant de choisir la colonne
            let column = players::get_column_choice();
    
            // Essayez de placer une pièce sur la grille
            match self.make_move(column) {
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
    }

    pub fn is_game_over(&self) -> bool {

        // Vérification des lignes horizontales
        for row in &self.grid {
            for (i, cell) in row.iter().enumerate() {
                if *cell != ' ' {
                    let symbol = *cell;
                    let mut count = 1;
                    for j in 1..4 {
                        if i + j < row.len() && row[i + j] == symbol {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count == 4 {
                        return true; // Il y a un gagnant
                    }
                }
            }
        }

        // Vérification des colonnes verticales
        for col in 0..self.grid[0].len() {
            for i in 0..self.grid.len() - 3 {
                let mut count = 0;
                for j in 0..4 {
                    if self.grid[i + j][col] == self.players[0].symbol.chars().next().unwrap() {
                        count += 1;
                    } else if self.grid[i + j][col] == self.players[1].symbol.chars().next().unwrap() {
                        count -= 1;
                    }
                }
                if count == 4 || count == -4 {
                    return true; // Il y a un gagnant
                }
            }
        }

        // Vérification des diagonales
        for row in 0..self.grid.len() - 3 {
            for col in 0..self.grid[0].len() - 3 {
                let mut count_diag1 = 0;
                let mut count_diag2 = 0;
                for i in 0..4 {
                    if self.grid[row + i][col + i] == self.players[0].symbol.chars().next().unwrap() {
                        count_diag1 += 1;
                    } else if self.grid[row + i][col + i] == self.players[1].symbol.chars().next().unwrap() {
                        count_diag1 -= 1;
                    }
                    if self.grid[row + i][col + 3 - i] == self.players[0].symbol.chars().next().unwrap() {
                        count_diag2 += 1;
                    } else if self.grid[row + i][col + 3 - i] == self.players[1].symbol.chars().next().unwrap() {
                        count_diag2 -= 1;
                    }
                }
                if count_diag1 == 4 || count_diag1 == -4 || count_diag2 == 4 || count_diag2 == -4 {
                    return true; // Il y a un gagnant
                }
            }
        }

        false
    }

    pub fn is_game_draw(&self) -> bool {

        for row in &self.grid {
            for cell in row {
                if *cell == ' ' {
                    // Il y a une case vide, le jeu n'est pas terminé
                    return false;
                }
            }
        }
        
        true
    }

}
