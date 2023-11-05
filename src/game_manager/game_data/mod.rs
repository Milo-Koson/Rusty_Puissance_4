mod grid;

mod players;

use crate::connect_4_error::Connect4Error;

//::{Player, IdPlayer, self}; 
use self::players::*;

/*
Structure qui reçoit la grille de jeu, les 2 joueurs, le joueur courant et le statut du jeu (fini ou non fini)
*/
pub struct GameData {
    pub grid: Vec<Vec<char>>,
    pub players: [Player; 2],
    pub current_player: usize,
    pub game_over: bool
}

impl GameData {

    /**
    Création d'une instance de la structure GameData qui initialise la grille, les noms des joueurs, le joueur courant
    et l'état du jeu (initialisé à non fini)
     */
    pub fn new() -> GameData {
        // On demande les noms des joueurs 
        let (player1_name, player2_name) = set_player_names();

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

    /**
    Fonction qui retourne le nom du joueur courant
     */
    pub fn get_current_player_name(&self) -> &str {
        &self.players[self.current_player].name
    }

    /**
    Fonction qui retourne le nom des joueurs
     */
    pub fn get_player_names(&self, n_player: i8) -> String {
        if n_player == 1 { self.players[0].name.to_string() } else { self.players[1].name.to_string() }
    }

    /**
    Implémentation dans GameData de la fonction display chargée d'afficher la grille de jeu
    */
    pub fn display(&self) {
        grid::display_grid(&self.grid);
    }

    /**
    Fonction ayant pour but de placer sur la grille de jeu le jeton du joueur courant
    */
    pub fn make_move(&mut self, column: usize) -> Result<(), Connect4Error> {
       
        if column >= self.grid[0].len() {
            self.display();
            return Err(Connect4Error::InvalidInput);
        }

        let row = self.grid.iter_mut().rev().find(|r| r[column] == ' ');

        if let Some(row) = row {

            // Place le jeton du joueur actuel dans la grille
            let current_player = &self.players[self.current_player];
            row[column] = current_player.symbol.chars().next().unwrap();
        
            // Actualise le joueur courant
            self.current_player = 1 - self.current_player;

            Ok(())
        }
        else {
            Err(Connect4Error::ColumnFull)
        }
    }      

    /**
    Fonction qui détermine les différentes étapes du jeu 
    1) On détermne le joueur courant
    2) On affiche la grille de jeu
    3) On attend que le joueur courant place son jeton sur la grille de jeu
    4) On vérifie que le jeton peut-être placé sur la grille
    */
    pub fn play_game(&mut self) -> Result<(), Connect4Error> {
        
        // Détermine le joueur courant
        let current_player = &self.players[self.current_player];
        
        // Affiche la grille de jeu
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
                    // Effacement de la grille de jeu pour actualiser le terminal
                    clearscreen::clear().expect("Échec de l'effacement de l'écran !");
                }
                Err(Connect4Error::InvalidInput) => {
                    return Err(Connect4Error::InvalidInput);
                }
                Err(connect_4_error) => {
                    return Err(connect_4_error);
                }
            }
        }
        Ok(())
    }

    /**
    Fonction qui vérifie les conditions de victoire à chaque fois qu'un jeton est placé
    Vérifie s'il y a 4 jetons alignés sur les lignes horizontales, verticales et diagonales
    */
    //Fonction is_game_over rédigée par ChatGPT
    pub fn is_game_over(&self) -> bool {
        let symbol_chars: Vec<char> = self.players.iter().map(|player| player.symbol.chars().next().unwrap()).collect();
        for row in &self.grid {
            if let Some(symbol) = row.iter().find(|&&cell| cell != ' ') {
                if row.windows(4).any(|window| window.iter().all(|&cell| cell == *symbol)) {
                    return true;
                }
            }
        }
    
        for col in 0..self.grid[0].len() {
            for i in 0..self.grid.len() - 3 {
                if (0..4).all(|j| {
                    let cell = self.grid[i + j][col];
                    symbol_chars.contains(&cell)
                }) {
                    return true;
                }
            }
        }
    
        for row in 0..self.grid.len() - 3 {
            for col in 0..self.grid[0].len() - 3 {
                if (0..4).all(|i| {
                    (0..4).all(|j| {
                        let cell1 = self.grid[row + i + j][col + i + j];
                        let cell2 = self.grid[row + i + j][col + 3 - i - j];
                        symbol_chars.contains(&cell1) || symbol_chars.contains(&cell2)
                    })
                }) {
                    return true;
                }
            }
        }
    
        false
    }

    /**
    Fonction qui vérifie si la grille est pleine, s'il n'y a pas 4 jetons similaires qui sont alignés et que la grille est pleine
    le match est déclaré comme nul
    */
    pub fn is_game_draw(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&cell| cell != ' '))
    }    

    /**
    Fonction qui détermine si la partie de jeu est finie
    */
    pub fn timeout(&mut self) {
        self.game_over = true;
    }

}
