mod accessor;
mod cell;
mod column;
mod constant;
mod grid;
mod line;
mod read;
mod square;

use constant::*;
use grid::*;
use read::*;
use std::io;

//ask the user and read his answer
fn read_u8(mess: String) -> Option<u8> {
    println!("{}", mess);
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
    let res = res.trim();

    let r: u8 = match res.parse() {
        Err(e) => {
            println!("erreur {}", e);
            return None;
        }
        Ok(v) => v,
    };
    Some(r)
}

//ask the user and read his answer
fn read_string(mess: String) -> String {
    println!("{}", mess);
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
    let res = res.trim();
    res.to_string()
}

fn manual() {
    let mut g = Grid::default();
    println!("resolved = {}", g.is_resolved());
    println!();
    g.display();
    loop {
        let l = read_u8("line?".to_string());
        if l.is_none() {
            continue;
        }
        let l = l.unwrap();
        let c = read_u8("column?".to_string());
        if c.is_none() {
            continue;
        }
        let c = c.unwrap();
        let v = read_u8("value?".to_string());
        if v.is_none() {
            continue;
        }
        let v = v.unwrap();
        g.set_val(l, c, v);
        println!();
        g.display();
        if g.check_puzzle() {
            println!("Sudoku resolved!");
            return;
        }
    }
}

pub fn resolve(g: &mut Grid, debug: bool) {
    g.display();
    //if already resolved...
    if ! g.is_resolved() {
        let mut step = 0;
        //loop until no more to solve
        loop {
            if debug {
                println!("--------------------step {}--------------------", step);
            }
            if !g.resolve() {
                break;
            }
            if debug {
                g.display();
            }
            step += 1;
        }    
    }
    g.display();
    if g.is_resolved() {
        println!("Grid resolved!!!!!");
    }
    if debug {
        g.debug();
    }
}

fn test_solving(debug: bool) {
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
    resolve(&mut g1, debug);
}

fn test_solving_easy(debug: bool) {
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

    resolve(&mut g1, debug);
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
    | 1  2  7 | 4  5  6 | 3  9  8 |
    | 8  3  4 | 2  9  1 | 7  5  6 |
    | 5  9  6 | 3  8  7 | 1  2  4 |
    -------------------------------
    | 4  7  5 | 1  3  9 | 8  6  2 |
    | 9  6  8 | 5  7  2 | 4  3  1 |
    | 2  1  3 | 6  4  8 | 5  7  9 |
    -------------------------------
    | 3  4  9 | 8  6  5 | 2  1  7 |
    | 7  5  1 | 9  2  4 | 6  8  3 |
    | 6  8  2 | 7  1  3 | 9  4  5 |
    -------------------------------
        */
}

fn test_solving_medium(debug: bool) {
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
    resolve(&mut g1, debug);
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
    | 5  3  ? | 2  4  ? | 1  8  6 |
    | 4  8  ? | ?  1  6 | 5  2  3 |
    | 1  6  2 | 8  5  3 | 7  9  4 |
    -------------------------------
    | 2  1  5 | ?  3  ? | 6  4  8 |
    | 6  9  8 | ?  2  ? | 3  7  1 |
    | 3  7  4 | 6  8  1 | 2  5  9 |
    -------------------------------
    | 9  4  1 | 3  7  2 | 8  6  5 |
    | 8  2  3 | ?  6  ? | 9  1  7 |
    | 7  5  6 | 1  9  8 | 4  3  2 |
    -------------------------------
    */
}

fn test_solving_difficult(debug: bool) {
    let mut g1 = Grid::default();

    g1.set_val(2, 1, 5);
    g1.set_val(2, 3, 2);
    g1.set_val(2, 4, 9);
    g1.set_val(2, 6, 8);

    g1.set_val(3, 1, 1);
    g1.set_val(3, 2, 6);
    g1.set_val(3, 4, 2);
    g1.set_val(3, 5, 3);

    g1.set_val(4, 3, 1);
    g1.set_val(4, 7, 7);
    g1.set_val(4, 9, 4);

    g1.set_val(5, 3, 4);
    g1.set_val(5, 5, 9);
    g1.set_val(5, 7, 3);

    g1.set_val(6, 1, 7);
    g1.set_val(6, 3, 8);
    g1.set_val(6, 7, 5);

    g1.set_val(7, 5, 8);
    g1.set_val(7, 6, 5);
    g1.set_val(7, 8, 6);
    g1.set_val(7, 9, 7);

    g1.set_val(8, 4, 6);
    g1.set_val(8, 6, 7);
    g1.set_val(8, 7, 8);
    g1.set_val(8, 9, 1);

    resolve(&mut g1, debug);
    /*
        Grid:
    -------------------------------
    | ?  ?  ? | ?  ?  ? | ?  ?  ? |
    | 5  ?  2 | 9  ?  8 | ?  ?  ? |
    | 1  6  ? | 2  3  ? | ?  ?  ? |
    -------------------------------
    | ?  ?  1 | ?  ?  ? | 7  ?  4 |
    | ?  ?  4 | ?  9  ? | 3  ?  ? |
    | 7  ?  8 | ?  ?  ? | 5  ?  ? |
    -------------------------------
    | ?  ?  ? | ?  8  5 | ?  6  7 |
    | ?  ?  ? | 6  ?  7 | 8  ?  1 |
    | ?  ?  ? | ?  ?  ? | ?  ?  ? |
    -------------------------------
        Solution found (17 steps)
    -------------------------------
    | 9  8  3 | 5  7  6 | 1  4  2 |
    | 5  4  2 | 9  1  8 | 6  7  3 |
    | 1  6  7 | 2  3  4 | 9  5  8 |
    -------------------------------
    | 6  9  1 | 8  5  3 | 7  2  4 |
    | 2  5  4 | 7  9  1 | 3  8  6 |
    | 7  3  8 | 4  6  2 | 5  1  9 |
    -------------------------------
    | 4  1  9 | 3  8  5 | 2  6  7 |
    | 3  2  5 | 6  4  7 | 8  9  1 |
    | 8  7  6 | 1  2  9 | 4  3  5 |
    -------------------------------
    */
}

fn test_solving_diabolical(debug: bool) {
    let mut g1 = Grid::default();

    g1.set_val(1, 2, 8);
    g1.set_val(1, 3, 3);
    g1.set_val(1, 4, 9);

    g1.set_val(2, 1, 5);

    g1.set_val(3, 4, 1);
    g1.set_val(3, 5, 4);
    g1.set_val(3, 8, 2);

    g1.set_val(4, 1, 3);
    g1.set_val(4, 3, 9);
    g1.set_val(4, 6, 8);
    g1.set_val(4, 7, 6);

    g1.set_val(5, 3, 7);
    g1.set_val(5, 7, 1);

    g1.set_val(6, 3, 4);
    g1.set_val(6, 4, 2);
    g1.set_val(6, 7, 3);
    g1.set_val(6, 9, 7);

    g1.set_val(7, 2, 4);
    g1.set_val(7, 5, 6);
    g1.set_val(7, 6, 3);

    g1.set_val(8, 9, 5);

    g1.set_val(9, 6, 4);
    g1.set_val(9, 7, 9);
    g1.set_val(9, 8, 3);

    resolve(&mut g1, debug);
    /*
        Grid:
    -------------------------------
    | ?  8  3 | 9  ?  ? | ?  ?  ? |
    | 5  ?  ? | ?  ?  ? | ?  ?  ? |
    | ?  ?  ? | 1  4  ? | ?  2  ? |
    -------------------------------
    | 3  ?  9 | ?  ?  8 | 6  ?  ? |
    | ?  ?  7 | ?  ?  ? | 1  ?  ? |
    | ?  ?  4 | 2  ?  ? | 3  ?  7 |
    -------------------------------
    | ?  4  ? | ?  6  3 | ?  ?  ? |
    | ?  ?  ? | ?  ?  ? | ?  ?  5 |
    | ?  ?  ? | ?  ?  4 | 9  3  ? |
    -------------------------------
        Solution not found
    -------------------------------
    | 4  8  3 | 9  ?  ? | 5  ?  ? |
    | 5  ?  ? | ?  ?  ? | ?  ?  ? |
    | 7  9  6 | 1  4  5 | 8  2  3 |
    -------------------------------
    | 3  ?  9 | ?  ?  8 | 6  ?  ? |
    | ?  ?  7 | ?  ?  ? | 1  ?  ? |
    | ?  ?  4 | 2  ?  ? | 3  ?  7 |
    -------------------------------
    | 9  4  ? | ?  6  3 | ?  ?  ? |
    | ?  3  ? | ?  ?  ? | ?  ?  5 |
    | ?  7  ? | ?  ?  4 | 9  3  ? |
    -------------------------------
    */
}

fn test_from_disk(debug: bool) {
    let fic = read_string("Filename?".to_string());
    let mut g1 = read(&fic);
    resolve(&mut g1, debug);
}

fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);
    let mut debug = true;
    loop {
        println!("0:toggle debugging (actual:{})", debug);
        println!("1:test solving");
        println!("2:test solving easy");
        println!("3:test solving medium");
        println!("4:test solving difficult");
        println!("5:test solving diabolical");
        println!("8:test a grid read from disk");
        println!("9:fill manualy");
        println!("99:quit");
        match read_u8("Your choice?".to_string()) {
            None => {
                continue;
            }
            Some(0) => {
                debug = !debug;
            }
            Some(1) => {
                test_solving(debug);
            }
            Some(2) => {
                test_solving_easy(debug);
            } //-> Ok
            Some(3) => {
                test_solving_medium(debug);
            } //-> not yet
            Some(4) => {
                test_solving_difficult(debug);
            } //-> not solved
            Some(5) => {
                test_solving_diabolical(debug);
            } //->
            Some(8) => {
                test_from_disk(debug);
            }
            Some(9) => {
                manual();
            } //->
            Some(99) => {
                return;
            } //->
            _ => {
                continue;
            }
        }
    }
}
