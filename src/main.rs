mod accessor;
mod cell;
mod constant;
mod grid;

use accessor::*;
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

fn test() {
    let mut g = Grid::new();
    g.display();
    g.set_val(1, 1, 1);
    g.set_val(1, 2, 2);
    g.set_val(1, 3, 3);
    g.set_val(1, 4, 4);
    g.set_val(1, 5, 5);
    g.set_val(1, 6, 6);
    g.set_val(1, 7, 7);
    g.set_val(1, 8, 8);
    g.set_val(1, 9, 9);
    g.set_val(2, 1, 4);
    g.set_val(2, 2, 5);
    g.set_val(2, 3, 6);
    g.set_val(2, 4, 7);
    g.set_val(2, 5, 8);
    g.set_val(2, 6, 9);
    g.set_val(2, 7, 1);
    g.set_val(2, 8, 2);
    g.set_val(2, 9, 3);
    g.set_val(3, 1, 7);
    g.set_val(3, 2, 8);
    g.set_val(3, 3, 9);
    g.set_val(3, 4, 1);
    g.set_val(3, 5, 2);
    g.set_val(3, 6, 3);
    g.set_val(3, 7, 4);
    g.set_val(3, 8, 5);
    g.set_val(3, 9, 6);
    g.set_val(4, 1, 2);
    g.set_val(4, 2, 3);
    g.set_val(4, 3, 4);
    g.set_val(4, 4, 5);
    g.set_val(4, 5, 6);
    g.set_val(4, 6, 7);
    g.set_val(4, 7, 8);
    g.set_val(4, 8, 9);
    g.set_val(4, 9, 1);
    g.set_val(5, 1, 5);
    g.set_val(5, 2, 6);
    g.set_val(5, 3, 7);
    g.set_val(5, 4, 8);
    g.set_val(5, 5, 9);
    g.set_val(5, 6, 1);
    g.set_val(5, 7, 2);
    g.set_val(5, 8, 3);
    g.set_val(5, 9, 4);
    g.set_val(6, 1, 8);
    g.set_val(6, 2, 9);
    g.set_val(6, 3, 1);
    g.set_val(6, 4, 2);
    g.set_val(6, 5, 3);
    g.set_val(6, 6, 4);
    g.set_val(6, 7, 5);
    g.set_val(6, 8, 6);
    g.set_val(6, 9, 7);
    g.set_val(7, 1, 3);
    g.set_val(7, 2, 4);
    g.set_val(7, 3, 5);
    g.set_val(7, 4, 6);
    g.set_val(7, 5, 7);
    g.set_val(7, 6, 8);
    g.set_val(7, 7, 9);
    g.set_val(7, 8, 1);
    g.set_val(7, 9, 2);
    g.set_val(8, 1, 6);
    g.set_val(8, 2, 7);
    g.set_val(8, 3, 8);
    g.set_val(8, 4, 9);
    g.set_val(8, 5, 1);
    g.set_val(8, 6, 2);
    g.set_val(8, 7, 3);
    g.set_val(8, 8, 4);
    g.set_val(8, 9, 5);
    g.set_val(9, 1, 9);
    g.set_val(9, 2, 1);
    g.set_val(9, 3, 2);
    g.set_val(9, 4, 3);
    g.set_val(9, 5, 4);
    g.set_val(9, 6, 5);
    g.set_val(9, 7, 6);
    g.set_val(9, 8, 7);
    g.set_val(9, 9, 8);
    g.check_puzzle();
    g.display();

    let a = Accessor::new();
}

fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);

    let mut g = Grid::new();
    println!("résolue = {}", g.is_resolved());
    println!();
    g.display();

    test();

    fill(&mut g);
}
