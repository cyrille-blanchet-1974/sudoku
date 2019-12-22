mod cell;
mod constant;
mod grid;

use constant::*;
use grid::*;

fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);

    let g = Grid::new();
    println!("résolue = {}", g.is_resolved());
}
