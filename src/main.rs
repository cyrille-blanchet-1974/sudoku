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
    let mut g1 = Grid::default();
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
    g1.display();//g1.debug();
    let r = resolve(&mut g1);
    if r {
        println!("grille résolue");
    }
    g1.display();//g1.debug();
}

fn test_solving_easy(){
    let mut g1 = Grid::default();
    g1.set_val(1, 1, 1);
    g1.set_val(1, 3, 7);

    g1.set_val(2, 3, 4);
    g1.set_val(2, 4, 2);
    g1.set_val(2, 5, 9);
    g1.set_val(2, 9, 6);

    g1.set_val(3, 2, 9);
    g1.set_val(3, 5, 8);
    g1.set_val(3, 6, 7);
    g1.set_val(3, 8, 2);
    g1.set_val(3, 9, 4);

    g1.set_val(4, 1, 4);
    g1.set_val(4, 2, 7);
    g1.set_val(4, 3, 5);
    g1.set_val(4, 4, 1);
    g1.set_val(4, 7, 8);
    g1.set_val(4, 8, 6);

    g1.set_val(6, 2, 1);
    g1.set_val(6, 3, 3);
    g1.set_val(6, 6, 8);
    g1.set_val(6, 7, 5);
    g1.set_val(6, 8, 7);
    g1.set_val(6, 9, 9);

    g1.set_val(7, 1, 3);
    g1.set_val(7, 2, 4);
    g1.set_val(7, 4, 8);
    g1.set_val(7, 5, 6);
    g1.set_val(7, 8, 1);

    g1.set_val(8, 1, 7);
    g1.set_val(8, 5, 2);
    g1.set_val(8, 6, 4);
    g1.set_val(8, 7, 6);

    g1.set_val(9, 7, 9);
    g1.set_val(9, 9, 5);
    g1.display();//g1.debug();
    let r = resolve(&mut g1);
    if r {
        println!("grille résolue");
    }
    g1.display();//g1.debug();
    /*
    Grid:
-------------------------------
| 1  ?  7 | ?  ?  ? | ?  ?  ? |
| ?  ?  4 | 2  9  ? | ?  ?  6 |
| ?  9  ? | ?  8  7 | ?  2  4 |
-------------------------------
| 4  7  5 | 1  ?  ? | 8  6  ? |
| ?  ?  ? | ?  ?  ? | ?  ?  ? |
| ?  1  3 | ?  ?  8 | 5  7  9 |
-------------------------------
| 3  4  ? | 8  6  ? | ?  1  ? |
| 7  ?  ? | ?  2  4 | 6  ?  ? |
| ?  ?  ? | ?  ?  ? | 9  ?  5 |
-------------------------------    
    Solution found
-------------------------------
| 1  2  7 | 4  5  2 | 3  9  8 |
| 8  3  4 | 2  9  1 | 7  5  6 |
| 5  9  6 | 3  8  7 | 1  2  4 |
-------------------------------
| 4  7  5 | 1  3  9 | 8  6  2 |
| 9  6  1 | 5  7  2 | 4  3  1 |
| 2  1  3 | 6  4  8 | 5  7  9 |
-------------------------------
| 3  4  9 | 8  6  5 | 2  1  7 |
| 7  5  1 | 9  2  4 | 6  8  3 |
| 6  8  2 | 7  1  3 | 9  4  5 |
-------------------------------

    */
}

fn test_solving_medium(){
    let mut g1 = Grid::default();
    g1.set_val(1, 1, 5);
    g1.set_val(1, 5, 4);

    g1.set_val(2, 2, 8);
    g1.set_val(2, 8, 2);
    g1.set_val(2, 9, 3);

    g1.set_val(3, 4, 8);
    g1.set_val(3, 5, 5);
    g1.set_val(3, 6, 3);
    g1.set_val(3, 7, 7);

    g1.set_val(4, 1, 2);
    g1.set_val(4, 7, 6);
    g1.set_val(4, 8, 4);

    g1.set_val(5, 1, 6);
    g1.set_val(5, 3, 8);
    g1.set_val(5, 7, 3);
    g1.set_val(5, 9, 1);

    g1.set_val(6, 2, 7);
    g1.set_val(6, 3, 4);
    g1.set_val(6, 9, 9);

    g1.set_val(7, 3, 1);
    g1.set_val(7, 4, 3);
    g1.set_val(7, 5, 7);
    g1.set_val(7, 6, 2);

    g1.set_val(8, 1, 8);
    g1.set_val(8, 3, 3);
    g1.set_val(8, 8, 1);

    g1.set_val(9, 5, 9);
    g1.set_val(9, 9, 2);
    g1.display();//g1.debug();
    let r = resolve(&mut g1);
    if r {
        println!("grille résolue");
    }
    g1.display();//g1.debug();
    /*
    Grid:
-------------------------------
| 5  ?  ? | ?  4  ? | ?  ?  ? |
| ?  8  ? | ?  ?  ? | ?  2  3 |
| ?  ?  ? | 8  5  3 | 7  ?  ? |
-------------------------------
| 2  ?  ? | ?  ?  ? | 6  4  ? |
| 6  ?  8 | ?  ?  ? | 3  ?  1 |
| ?  7  4 | ?  ?  ? | ?  ?  9 |
-------------------------------
| ?  ?  1 | 3  7  2 | ?  ?  ? |
| 8  ?  3 | ?  ?  ? | ?  1  ? |
| ?  ?  ? | ?  9  ? | ?  ?  2 |
-------------------------------  
    Solution not found
-------------------------------
| 5  ?  ? | ?  3  ? | ?  ?  ? |   <- 1 in col 7 not found because lvl 2 need 1 found line 3
| ?  8  ? | ?  1  ? | ?  2  3 |   as 3 is already used by 7 we could found => lvl 3 is there!!!
| ?  ?  ? | 8  5  3 | 7  ?  ? |
-------------------------------
| 2  ?  ? | ?  8  ? | 6  4  7 |
| 6  ?  8 | ?  2  ? | 3  7  1 |
| ?  7  4 | ?  8  ? | 5  5  9 |
-------------------------------
| ?  ?  1 | 3  7  2 | ?  ?  ? |
| 8  ?  3 | ?  6  ? | 2  1  ? |
| ?  ?  ? | ?  9  ? | 1  3  2 |
-------------------------------
    */
}


fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);

    test_solving();

    test_solving_easy(); //-> Ok

    test_solving_medium();//->


    let mut g = Grid::default();
    println!("résolue = {}", g.is_resolved());
    println!();
    g.display();
    fill(&mut g);
}
