mod accessor;
mod cell;
mod column;
mod constant;
mod grid;
mod line;
mod read;
mod resolver;
mod resolver_lvl1;
mod resolver_lvl2;
mod resolver_lvl3;
mod resolver_lvl4;
mod square;

use cell::CellType;
use constant::*;
use grid::*;
use read::*;
use resolver::*;
use std::io;
use std::time::SystemTime;

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

fn manual(debug: bool, display: bool) {
    let mut g = Grid::default();
    println!("resolved = {}", g.is_resolved());
    println!();
    g.display();
    loop {
        println!("[1-{}] 0 to solve", MAX);
        let l = read_u8("Line?".to_string());
        if l.is_none() {
            continue;
        }
        let l = l.unwrap();
        //control >=0 useless ecause type is u8 unsigned 8
        if l > MAX {
            continue;
        }
        if l == 0 {
            resolve(&mut g, debug, display);
            return;
        }
        println!("[1-{}] 0 to solve", MAX);
        let c = read_u8("Column?".to_string());
        if c.is_none() {
            continue;
        }
        let c = c.unwrap();
        if c > MAX {
            continue;
        }
        if c == 0 {
            resolve(&mut g, debug, display);
            return;
        }
        println!("[1-{}] 0 to solve", MAX);
        let v = read_u8("Value?".to_string());
        if v.is_none() {
            continue;
        }
        let v = v.unwrap();
        if v > MAX {
            continue;
        }
        if v == 0 {
            resolve(&mut g, debug, display);
            return;
        }
        g.set_val(l, c, v, CellType::Origin);
        println!();
        g.display();
        if !g.is_valid() {
            println!("Sudoku invalid!");
            return;
        }
        if g.is_resolved() {
            println!("Sudoku resolved!");
            return;
        }
    }
}

pub fn resolve(g: &mut Grid, debug: bool, display: bool) -> bool {
    let mut r = Resolver::new(debug, display);
    println!("****Initial data for the grid****");
    g.display();
    let start_elapse = SystemTime::now();
    //let mut tps = Duration::new(0, 0);
    let res = r.go(g);
    let end = SystemTime::now();
    let tps = end
        .duration_since(start_elapse)
        .expect("ERROR computing duration!");
    println!("Duration={:?}", tps);

    println!("****Final data for the grid****");
    g.display();
    g.legend();
    println!("Grid resolved!!!!!");
    r.display_stats();
    res
}

fn test_solving(debug: bool, display: bool) -> bool {
    println!("1->resolution test!");
    let v = vec![
        "1,?,?,?,?,?,?,?,?".to_string(),
        "?,?,?,2,1,?,?,?,?".to_string(),
        "?,?,?,?,?,?,3,?,?".to_string(),
        "?,4,?,?,?,?,?,?,?".to_string(),
        "?,?,?,?,5,?,?,1,?".to_string(),
        "?,?,?,?,?,?,?,6,?".to_string(),
        "?,?,7,?,?,?,?,?,?".to_string(),
        "?,?,?,?,?,8,?,?,?".to_string(),
        "?,?,?,?,?,?,1,?,9".to_string(),
    ];

    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 43 steps (26 guesses all good) // 74 steps 29 guess 1 bad
    -------------------------------                       -------------------------------
    | 1  ?  ? | ?  ?  ? | ?  ?  ? |                       | 1  2  3 | 4  6  5 | 7  9  8 |
    | ?  ?  ? | 2  1  ? | ?  ?  ? |                       | 7  8  9 | 2  1  3 | 4  5  6 |
    | ?  ?  ? | ?  ?  ? | 3  ?  ? |                       | 4  5  6 | 7  8  9 | 3  2  1 |
    -------------------------------                       -------------------------------
    | ?  4  ? | ?  ?  ? | ?  ?  ? |                       | 2  4  1 | 6  9  7 | 5  8  3 |
    | ?  ?  ? | ?  5  ? | ?  1  ? |                       | 6  7  8 | 3  5  2 | 9  1  4 |
    | ?  ?  ? | ?  ?  ? | ?  6  ? |                       | 3  9  5 | 8  4  1 | 2  6  7 |
    -------------------------------                       -------------------------------
    | ?  ?  7 | ?  ?  ? | ?  ?  ? |                       | 5  1  7 | 9  3  6 | 8  4  2 |
    | ?  ?  ? | ?  ?  8 | ?  ?  ? |                       | 9  3  4 | 1  2  8 | 6  7  5 |
    | ?  ?  ? | ?  ?  ? | 1  ?  9 |                       | 8  6  2 | 5  7  4 | 1  3  9 |
    -------------------------------                       -------------------------------
    */
}

fn test_solving_easy(debug: bool, display: bool) -> bool {
    println!("2->resolution easy!");
    let v = vec![
        "1,?,7,?,?,?,?,?,?".to_string(),
        "?,?,4,2,9,?,?,?,6".to_string(),
        "?,9,?,?,8,7,?,2,4".to_string(),
        "4,7,5,1,?,?,8,6,?".to_string(),
        "?,?,?,?,?,?,?,?,?".to_string(),
        "?,1,3,?,?,8,5,7,9".to_string(),
        "3,4,?,8,6,?,?,1,?".to_string(),
        "7,?,?,?,2,4,6,?,?".to_string(),
        "?,?,?,?,?,?,9,?,5".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved 3 steps (0 guess)
    -------------------------------                           -------------------------------
    | 1  ?  7 | ?  ?  ? | ?  ?  ? |                           | 1  2  7 | 4  5  6 | 3  9  8 |
    | ?  ?  4 | 2  9  ? | ?  ?  6 |                           | 8  3  4 | 2  9  1 | 7  5  6 |
    | ?  9  ? | ?  8  7 | ?  2  4 |                           | 5  9  6 | 3  8  7 | 1  2  4 |
    -------------------------------                           -------------------------------
    | 4  7  5 | 1  ?  ? | 8  6  ? |                           | 4  7  5 | 1  3  9 | 8  6  2 |
    | ?  ?  ? | ?  ?  ? | ?  ?  ? |                           | 9  6  8 | 5  7  2 | 4  3  1 |
    | ?  1  3 | ?  ?  8 | 5  7  9 |                           | 2  1  3 | 6  4  8 | 5  7  9 |
    -------------------------------                           -------------------------------
    | 3  4  ? | 8  6  ? | ?  1  ? |                           | 3  4  9 | 8  6  5 | 2  1  7 |
    | 7  ?  ? | ?  2  4 | 6  ?  ? |                           | 7  5  1 | 9  2  4 | 6  8  3 |
    | ?  ?  ? | ?  ?  ? | 9  ?  5 |                           | 6  8  2 | 7  1  3 | 9  4  5 |
    -------------------------------                           -------------------------------*/
}

fn test_solving_medium(debug: bool, display: bool) -> bool {
    println!("3->resolution medium!");
    let v = vec![
        "5,?,?,?,4,?,?,?,?".to_string(),
        "?,8,?,?,?,?,?,2,3".to_string(),
        "?,?,?,8,5,3,7,?,?".to_string(),
        "2,?,?,?,?,?,6,4,?".to_string(),
        "6,?,8,?,?,?,3,?,1".to_string(),
        "?,7,4,?,?,?,?,?,9".to_string(),
        "?,?,1,3,7,2,?,?,?".to_string(),
        "8,?,3,?,?,?,?,1,?".to_string(),
        "?,?,?,?,9,?,?,?,2".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 9 steps (2 guesses All goods) // 10 steps 2 guesses 0 bad
    -------------------------------                           -------------------------------
    | 5  ?  ? | ?  4  ? | ?  ?  ? |                           | 5  3  7 | 2  4  9 | 1  8  6 |
    | ?  8  ? | ?  ?  ? | ?  2  3 |                           | 4  8  9 | 7  1  6 | 5  2  3 |
    | ?  ?  ? | 8  5  3 | 7  ?  ? |                           | 1  6  2 | 8  5  3 | 7  9  4 |
    -------------------------------                           -------------------------------
    | 2  ?  ? | ?  ?  ? | 6  4  ? |                           | 2  1  5 | 9  3  7 | 6  4  8 |
    | 6  ?  8 | ?  ?  ? | 3  ?  1 |                           | 6  9  8 | 4  2  5 | 3  7  1 |
    | ?  7  4 | ?  ?  ? | ?  ?  9 |                           | 3  7  4 | 6  8  1 | 2  5  9 |
    -------------------------------                           -------------------------------
    | ?  ?  1 | 3  7  2 | ?  ?  ? |                           | 9  4  1 | 3  7  2 | 8  6  5 |
    | 8  ?  3 | ?  ?  ? | ?  1  ? |                           | 8  2  3 | 5  6  4 | 9  1  7 |
    | ?  ?  ? | ?  9  ? | ?  ?  2 |                           | 7  5  6 | 1  9  8 | 4  3  2 |
    -------------------------------                           -------------------------------*/
}

fn test_solving_difficult(debug: bool, display: bool) -> bool {
    println!("4->resolution difficult!");
    let v = vec![
        "?,?,?,?,?,?,?,?,?".to_string(),
        "5,?,2,9,?,8,?,?,?".to_string(),
        "1,6,?,2,3,?,?,?,?".to_string(),
        "?,?,1,?,?,?,7,?,4".to_string(),
        "?,?,4,?,9,?,3,?,?".to_string(),
        "7,?,8,?,?,?,5,?,?".to_string(),
        "?,?,?,?,8,5,?,6,7".to_string(),
        "?,?,?,6,?,7,8,?,1".to_string(),
        "?,?,?,?,?,?,?,?,?".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 9 steps (0 guess)
    -------------------------------                           -------------------------------
    | ?  ?  ? | ?  ?  ? | ?  ?  ? |                           | 9  8  3 | 5  7  6 | 1  4  2 |
    | 5  ?  2 | 9  ?  8 | ?  ?  ? |                           | 5  4  2 | 9  1  8 | 6  7  3 |
    | 1  6  ? | 2  3  ? | ?  ?  ? |                           | 1  6  7 | 2  3  4 | 9  5  8 |
    -------------------------------                           -------------------------------
    | ?  ?  1 | ?  ?  ? | 7  ?  4 |                           | 6  9  1 | 8  5  3 | 7  2  4 |
    | ?  ?  4 | ?  9  ? | 3  ?  ? |                           | 2  5  4 | 7  9  1 | 3  8  6 |
    | 7  ?  8 | ?  ?  ? | 5  ?  ? |                           | 7  3  8 | 4  6  2 | 5  1  9 |
    -------------------------------                           -------------------------------
    | ?  ?  ? | ?  8  5 | ?  6  7 |                           | 4  1  9 | 3  8  5 | 2  6  7 |
    | ?  ?  ? | 6  ?  7 | 8  ?  1 |                           | 3  2  5 | 6  4  7 | 8  9  1 |
    | ?  ?  ? | ?  ?  ? | ?  ?  ? |                           | 8  7  6 | 1  2  9 | 4  3  5 |
    -------------------------------                           -------------------------------*/
}

fn test_solving_diabolical(debug: bool, display: bool) -> bool {
    println!("5->resolution diabolic!");
    let v = vec![
        "?,8,3,9,?,?,?,?,?".to_string(),
        "5,?,?,?,?,?,?,?,?".to_string(),
        "?,?,?,1,4,?,?,2,?".to_string(),
        "3,?,9,?,?,8,6,?,?".to_string(),
        "?,?,7,?,?,?,1,?,?".to_string(),
        "?,?,4,2,?,?,3,?,7".to_string(),
        "?,4,?,?,6,3,?,?,?".to_string(),
        "?,?,?,?,?,?,?,?,5".to_string(),
        "?,?,?,?,?,4,9,3,?".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 10 steps (1 good guess)  // 12 steps 1 good guess
    -------------------------------                           -------------------------------
    | ?  8  3 | 9  ?  ? | ?  ?  ? |                           | 4  8  3 | 9  2  7 | 5  1  6 |
    | 5  ?  ? | ?  ?  ? | ?  ?  ? |                           | 5  2  1 | 3  8  6 | 7  4  9 |
    | ?  ?  ? | 1  4  ? | ?  2  ? |                           | 7  9  6 | 1  4  5 | 8  2  3 |
    -------------------------------                           -------------------------------
    | 3  ?  9 | ?  ?  8 | 6  ?  ? |                           | 3  1  9 | 4  7  8 | 6  5  2 |
    | ?  ?  7 | ?  ?  ? | 1  ?  ? |                           | 2  5  7 | 6  3  9 | 1  8  4 |
    | ?  ?  4 | 2  ?  ? | 3  ?  7 |                           | 8  6  4 | 2  5  1 | 3  9  7 |
    -------------------------------                           -------------------------------
    | ?  4  ? | ?  6  3 | ?  ?  ? |                           | 9  4  5 | 8  6  3 | 2  7  1 |
    | ?  ?  ? | ?  ?  ? | ?  ?  5 |                           | 1  3  8 | 7  9  2 | 4  6  5 |
    | ?  ?  ? | ?  ?  4 | 9  3  ? |                           | 6  7  2 | 5  1  4 | 9  3  8 |
    -------------------------------                           -------------------------------*/
}

fn test_solving_highest(debug: bool, display: bool) -> bool {
    println!("6->resolution highest");
    let v = vec![
        "1,?,?,?,?,7,?,9,?".to_string(),
        "?,3,?,?,2,?,?,?,8".to_string(),
        "?,?,9,6,?,?,5,?,?".to_string(),
        "?,?,5,3,?,?,9,?,?".to_string(),
        "?,1,?,?,8,?,?,?,2".to_string(),
        "6,?,?,?,?,4,?,?,?".to_string(),
        "3,?,?,?,?,?,?,1,?".to_string(),
        "?,4,?,?,?,?,?,?,7".to_string(),
        "?,?,7,?,?,?,3,?,?".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 46 steps (11 guesses, 6 wrongs and 5 goods)  //132 steps  25 guess  19 bads
    -------------------------------                           -------------------------------
    | 1  ?  ? | ?  ?  7 | ?  9  ? |                           | 1  6  2 | 8  5  7 | 4  9  3 |
    | ?  3  ? | ?  2  ? | ?  ?  8 |                           | 5  3  4 | 1  2  9 | 6  7  8 |
    | ?  ?  9 | 6  ?  ? | 5  ?  ? |                           | 7  8  9 | 6  4  3 | 5  2  1 |
    -------------------------------                           -------------------------------
    | ?  ?  5 | 3  ?  ? | 9  ?  ? |                           | 4  7  5 | 3  1  2 | 9  8  6 |
    | ?  1  ? | ?  8  ? | ?  ?  2 |                           | 9  1  3 | 5  8  6 | 7  4  2 |
    | 6  ?  ? | ?  ?  4 | ?  ?  ? |                           | 6  2  8 | 7  9  4 | 1  3  5 |
    -------------------------------                           -------------------------------
    | 3  ?  ? | ?  ?  ? | ?  1  ? |                           | 3  5  6 | 4  7  8 | 2  1  9 |
    | ?  4  ? | ?  ?  ? | ?  ?  7 |                           | 2  4  1 | 9  3  5 | 8  6  7 |
    | ?  ?  7 | ?  ?  ? | 3  ?  ? |                           | 8  9  7 | 2  6  1 | 3  5  4 |
    -------------------------------                           -------------------------------*/
}

fn test_solving_mindless(debug: bool, display: bool) -> bool {
    println!("7->resolution mindless");
    let v = vec![
        "1,?,?,?,?,?,?,?,2".to_string(),
        "?,9,?,4,?,?,?,5,?".to_string(),
        "?,?,6,?,?,?,7,?,?".to_string(),
        "?,5,?,9,?,3,?,?,?".to_string(),
        "?,?,?,?,7,?,?,?,?".to_string(),
        "?,?,?,8,5,?,?,4,?".to_string(),
        "7,?,?,?,?,?,6,?,?".to_string(),
        "?,3,?,?,?,9,?,8,?".to_string(),
        "?,?,2,?,?,?,?,?,1".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 487 steps (95 guesses, 87 wrongs and 9 goods)
    chomebook:Duration=  89.336526ms (release)    904.471428ms (debug)
    XPS17 = 86.0264ms (release)   1.0990636s (debug)
    //31 steps 10 good guesses
    -------------------------------                           -------------------------------
    | 1  ?  ? | ?  ?  ? | ?  ?  2 |                           | 1  7  4 | 3  8  5 | 9  6  2 |
    | ?  9  ? | 4  ?  ? | ?  5  ? |                           | 2  9  3 | 4  6  7 | 1  5  8 |
    | ?  ?  6 | ?  ?  ? | 7  ?  ? |                           | 5  8  6 | 1  9  2 | 7  3  4 |
    -------------------------------                           -------------------------------
    | ?  5  ? | 9  ?  3 | ?  ?  ? |                           | 4  5  1 | 9  2  3 | 8  7  6 |
    | ?  ?  ? | ?  7  ? | ?  ?  ? |                           | 9  2  8 | 6  7  4 | 3  1  5 |
    | ?  ?  ? | 8  5  ? | ?  4  ? |                           | 3  6  7 | 8  5  1 | 2  4  9 |
    -------------------------------                           -------------------------------
    | 7  ?  ? | ?  ?  ? | 6  ?  ? |                           | 7  1  9 | 5  4  8 | 6  2  3 |
    | ?  3  ? | ?  ?  9 | ?  8  ? |                           | 6  3  5 | 2  1  9 | 4  8  7 |
    | ?  ?  2 | ?  ?  ? | ?  ?  1 |                           | 8  4  2 | 7  3  6 | 5  9  1 |
    -------------------------------                           -------------------------------*/
}

fn test_solving_hardest(debug: bool, display: bool) -> bool {
    println!("7->resolution hardest");
    let v = vec![
        "8,?,?,?,?,?,?,?,?".to_string(),
        "?,?,3,6,?,?,?,?,?".to_string(),
        "?,7,?,?,9,?,2,?,?".to_string(),
        "?,5,?,?,?,7,?,?,?".to_string(),
        "?,?,?,?,4,5,7,?,?".to_string(),
        "?,?,?,1,?,?,?,3,?".to_string(),
        "?,?,1,?,?,?,?,6,8".to_string(),
        "?,?,8,5,?,?,?,1,?".to_string(),
        "?,9,?,?,?,?,4,?,?".to_string(),
    ];
    let mut g1 = from_vec(v, debug);
    resolve(&mut g1, debug, display)
    /*Solved in 487 steps (95 guesses, 87 wrongs and 9 goods)
    chomebook:Duration=  89.336526ms (release)    904.471428ms (debug)
    XPS17 = 86.0264ms (release)   1.0990636s (debug)
    //31 steps 10 good guesses
    -------------------------------                           -------------------------------
    | 1  ?  ? | ?  ?  ? | ?  ?  2 |                           | 1  7  4 | 3  8  5 | 9  6  2 |
    | ?  9  ? | 4  ?  ? | ?  5  ? |                           | 2  9  3 | 4  6  7 | 1  5  8 |
    | ?  ?  6 | ?  ?  ? | 7  ?  ? |                           | 5  8  6 | 1  9  2 | 7  3  4 |
    -------------------------------                           -------------------------------
    | ?  5  ? | 9  ?  3 | ?  ?  ? |                           | 4  5  1 | 9  2  3 | 8  7  6 |
    | ?  ?  ? | ?  7  ? | ?  ?  ? |                           | 9  2  8 | 6  7  4 | 3  1  5 |
    | ?  ?  ? | 8  5  ? | ?  4  ? |                           | 3  6  7 | 8  5  1 | 2  4  9 |
    -------------------------------                           -------------------------------
    | 7  ?  ? | ?  ?  ? | 6  ?  ? |                           | 7  1  9 | 5  4  8 | 6  2  3 |
    | ?  3  ? | ?  ?  9 | ?  8  ? |                           | 6  3  5 | 2  1  9 | 4  8  7 |
    | ?  ?  2 | ?  ?  ? | ?  ?  1 |                           | 8  4  2 | 7  3  6 | 5  9  1 |
    -------------------------------                           -------------------------------*/
}

fn resolve_from_disk(fic: String, debug: bool, display: bool) -> bool {
    let mut g1 = read(&fic, debug);
    println!("8->resolution from file {}!", fic);
    resolve(&mut g1, debug, display)
}

fn test_from_disk(debug: bool, display: bool) -> bool {
    let fic = read_string("Filename?".to_string());
    resolve_from_disk(fic, debug, display)
}

fn main() {
    println!("Sudoku resolution!");
    println!("size = {}x{}", LINESIZE, COLUMNSIZE);
    let mut debug = false;
    let mut display = false;
    loop {
        println!("1:test solving");
        println!("2:test solving easy");
        println!("3:test solving medium");
        println!("4:test solving difficult");
        println!("5:test solving diabolical");
        println!("6:test solving highest");
        println!("7:test solving mindless");
        println!("8:test solving hardest");
        println!("9:test a grid read from disk");
        println!("10:fill manualy");
        println!("97:toggle debugging (actual:{})", debug);
        println!("98:toggle display (actual:{})", display);
        println!("99:quit");
        match read_u8("Your choice?".to_string()) {
            None => {
                continue;
            }
            Some(97) => {
                debug = !debug;
            }
            Some(98) => {
                display = !display;
            }
            Some(1) => {
                test_solving(debug, display);
            }
            Some(2) => {
                test_solving_easy(debug, display);
            }
            Some(3) => {
                test_solving_medium(debug, display);
            }
            Some(4) => {
                test_solving_difficult(debug, display);
            }
            Some(5) => {
                test_solving_diabolical(debug, display);
            }
            Some(6) => {
                test_solving_highest(debug, display);
            }
            Some(7) => {
                test_solving_mindless(debug, display);
            }
            Some(8) => {
                test_solving_hardest(debug, display);
            }
            Some(9) => {
                test_from_disk(debug, display);
            }
            Some(10) => {
                manual(debug, display);
            }
            Some(99) => {
                println!("Sudoku resolution End!");
                return;
            }
            _ => {
                continue;
            }
        }
    }
}

#[test]
fn resolve_test() {
    assert_eq!(true, test_solving(false, false));
    assert_eq!(true, test_solving_easy(false, false));
    assert_eq!(true, test_solving_medium(false, false));
    assert_eq!(true, test_solving_difficult(false, false));
    assert_eq!(true, test_solving_diabolical(false, false));
    assert_eq!(true, test_solving_highest(false, false));
    assert_eq!(true, test_solving_mindless(false, false));
    assert_eq!(true, test_solving_hardest(false, false));
    assert_eq!(
        true,
        resolve_from_disk("test/easy.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/medium.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/difficult.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/diabolic.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/pascal.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/pascal2.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/pascal3.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/m.txt".to_string(), false, false)
    );
    assert_eq!(
        true,
        resolve_from_disk("test/hardest.txt".to_string(), false, false)
    );
}
