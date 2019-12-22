mod cell;
mod constant;
mod grid;

use constant::*;
use grid::*;
use std::io;

//ask the user and read his guess
fn read_u8(mess: String) -> u8 {
    println!("{}", mess);
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
    let res = res.trim();

    let r: u8 = match res.parse() {
        Err(e) => {
            println!("erreur {}", e);
            0
        }
        Ok(v) => v,
    };
    r
}

fn fill(g: &mut Grid) {
    loop {
        let l = read_u8("line?".to_string());
        let c = read_u8("column?".to_string());
        let v = read_u8("value?".to_string());
        g.set_val(l, c, v);
        println!();
        g.display();
    }
}

fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);

    let mut g = Grid::new();
    println!("résolue = {}", g.is_resolved());
    println!();
    g.display();

    fill(&mut g);
}
