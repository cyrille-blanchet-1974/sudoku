use super::cell::CellType;
use super::constant::*;
use super::grid::*;
use super::read::*;
use super::ui::*;

fn manual() -> Option<Grid> {
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
        //control >=0 useless because type is u8 unsigned 8
        if l > MAX {
            continue;
        }
        if l == 0 {
            return Some(g);
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
            return Some(g);
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
            return Some(g);
        }
        g.set_val(l, c, v, CellType::Origin);
        println!();
        g.display();
        if !g.is_valid() {
            println!("Sudoku invalid!");
            return None;
        }
        if g.is_resolved() {
            println!("Sudoku resolved!");
            return Some(g);
        }

    }
    //None
}

pub fn sample(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn easy(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn medium(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn difficult(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn diabolical(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn highest(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn mindless(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn hardest(debug: bool) -> Grid {
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
    from_vec(v, debug)
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

pub fn from_disk(fic: String, debug: bool) -> Grid {
    read(&fic, debug)
}

pub fn choose_grid(debug: bool) -> Option<Grid> {
    println!("Choose your grid");
    loop {
        println!("1:sample");
        println!("2:easy");
        println!("3:medium");
        println!("4:difficult");
        println!("5:diabolical");
        println!("6:highest");
        println!("7:mindless");
        println!("8:hardest");
        println!("9:grid read from disk");
        println!("10:fill manualy");
        println!("99:quit");
        match read_u8("Your choice?".to_string()) {
            None => {
                continue;
            }
            Some(1) => {
                return Some(sample(debug));
            }
            Some(2) => {
                return Some(easy(debug));
            }
            Some(3) => {
                return Some(medium(debug));
            }
            Some(4) => {
                return Some(difficult(debug));
            }
            Some(5) => {
                return Some(diabolical(debug));
            }
            Some(6) => {
                return Some(highest(debug));
            }
            Some(7) => {
                return Some(mindless(debug));
            }
            Some(8) => {
                return Some(hardest(debug));
            }
            Some(9) => {
                let fic = read_string("Filename?".to_string());
                return Some(from_disk(fic, debug));
            }
            Some(10) => {
                return manual();
            }
            Some(99) => {
                println!("Sudoku resolution End!");
                return None;
            }
            _ => {
                continue;
            }
        }
    }
}
