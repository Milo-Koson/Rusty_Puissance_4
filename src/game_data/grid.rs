pub fn create_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row = vec![' '; cols];
        grid.push(row);
    }
    grid
}

pub fn display_grid(grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Ligne supérieure
    print!("+");
    for _ in 0..cols {
        print!("---+");
    }
    println!();
    
    for row in grid {
        // Ligne de la grille
        print!("|");
        for cell in row {
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
}