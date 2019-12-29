mod accessor;
mod cell;
mod constant;
mod grid;
mod line;
mod column;
mod square;

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
        if g.check_puzzle() {
            println!("Sudoku resolved!");        
            return;
        }
    }
}

pub fn resolve(g:&mut Grid) -> bool
{
    //if already resolved...
    if g.is_resolved(){
        return true;
    }
    //loop until no more to solve
    loop{
        if !g.resolve_lvl1() && !g.resolve_lvl2(){ break;}
    }    
    g.is_resolved()
}

fn test_solving(){
    let mut g1 = Grid::new();
    g1.set_val(1, 1, 1);
    g1.set_val(2, 4, 2);
    g1.set_val(3, 7, 3);
    g1.set_val(4, 2, 4);
    g1.set_val(5, 5, 5);
    g1.set_val(6, 8, 6);
    g1.set_val(7, 3, 7);
    g1.set_val(8, 6, 8);
    g1.set_val(9, 9, 9);
    g1.set_val(2, 5, 1);
    g1.set_val(5, 8, 1);
    g1.set_val(9, 7, 1);    
    g1.display();g1.debug();
    let r = resolve(&mut g1);
    if r {
        println!("grille résolue");
    }
    g1.display();g1.debug();
}

fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);

    test_solving();


    let mut g = Grid::new();
    println!("résolue = {}", g.is_resolved());
    println!();
    g.display();
    fill(&mut g);
}
