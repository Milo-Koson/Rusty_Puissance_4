/**
Fonction qui permet la création d'une grille de jeu
 */

pub fn create_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row = vec![' '; cols];
        grid.push(row);
    }
    grid
}

/**
Fonction qui affiche la grille de jeu
*/
pub fn display_grid(grid: &Vec<Vec<char>>) {
    let cols = grid[0].len();
    
    // Ligne supérieure
    print!("+");
    for _ in 0..cols {
        print!("---+");
    }
    println!();
    
    for rows in grid {
        // Ligne de la grille
        print!("|");
        for cell in rows {
            print!(" {} |", cell);
        }
        println!();
    
        // Ligne séparatrice
        print!("+");
        for _ in 0..cols {
            print!("---+");
        }
        
        println!();
    }

    for col_num in 1..cols+1 {
        print!(" {}  ", col_num);
    }

    println!("\n");
}