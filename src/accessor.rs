use super::constant::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub enum Cardinal {
    N,
    S,
    E,
    W,
    C,
    NE,
    NW,
    SE,
    SW,
    Unknown,
}
impl Cardinal {
    pub fn get_value(self) -> u8 {
        match self {
            Cardinal::NW => 1,
            Cardinal::N => 2,
            Cardinal::NE => 3,
            Cardinal::W => 4,
            Cardinal::C => 5,
            Cardinal::E => 6,
            Cardinal::SW => 7,
            Cardinal::S => 8,
            Cardinal::SE => 9,
            Cardinal::Unknown => 0,
        }
    }

    pub fn from(self, val: u8) -> Cardinal {
        match val {
            1 => Cardinal::NW,
            2 => Cardinal::N,
            3 => Cardinal::NE,
            4 => Cardinal::W,
            5 => Cardinal::C,
            6 => Cardinal::E,
            7 => Cardinal::SW,
            8 => Cardinal::S,
            9 => Cardinal::SE,
            _ => Cardinal::Unknown, //default
        }
    }
    /*
     get coord of square ((line,column) ,(line,column))
    */
    pub fn get_coord(self) -> ((u8, u8), (u8, u8)) {
        match self {
            Cardinal::NW => ((1, 1), (3, 3)),
            Cardinal::N => ((4, 1), (6, 3)),
            Cardinal::NE => ((7, 1), (9, 3)),
            Cardinal::W => ((1, 4), (3, 6)),
            Cardinal::C => ((4, 4), (6, 6)),
            Cardinal::E => ((7, 4), (9, 6)),
            Cardinal::SW => ((1, 7), (3, 9)),
            Cardinal::S => ((4, 7), (6, 9)),
            Cardinal::SE => ((7, 7), (9, 9)),
            Cardinal::Unknown => ((0, 0), (0, 0)),
        }
    }
    /*
     get other square of the same line/row
    */
    pub fn _get_other(self) -> Vec<Cardinal> {
        match self {
            Cardinal::NW => vec![Cardinal::N, Cardinal::NE, Cardinal::W, Cardinal::SW],
            Cardinal::N => vec![Cardinal::NW, Cardinal::NE, Cardinal::C, Cardinal::S],
            Cardinal::NE => vec![Cardinal::NW, Cardinal::N, Cardinal::E, Cardinal::SE],
            Cardinal::W => vec![Cardinal::NW, Cardinal::SW, Cardinal::C, Cardinal::E],
            Cardinal::C => vec![Cardinal::N, Cardinal::S, Cardinal::W, Cardinal::E],
            Cardinal::E => vec![Cardinal::NE, Cardinal::SE, Cardinal::C, Cardinal::W],
            Cardinal::SW => vec![Cardinal::NW, Cardinal::W, Cardinal::S, Cardinal::SE],
            Cardinal::S => vec![Cardinal::SW, Cardinal::SE, Cardinal::C, Cardinal::N],
            Cardinal::SE => vec![Cardinal::SW, Cardinal::S, Cardinal::E, Cardinal::NE],
            Cardinal::Unknown => Vec::new(),
        }
    }
    /*
    get all squares
    */
    pub fn get_all(self) -> Vec<Cardinal> {
        vec![
            Cardinal::NW,
            Cardinal::N,
            Cardinal::NE,
            Cardinal::W,
            Cardinal::C,
            Cardinal::S,
            Cardinal::SW,
            Cardinal::S,
            Cardinal::SE,
        ]
    }
    /*
     get lines of a square
    */
    /*
     get coord of square ((line,column) ,(line,column))
    */
    pub fn get_lines(self) -> Vec<u8> {
        match self {
            Cardinal::NW => vec![1, 2, 3],
            Cardinal::N => vec![1, 2, 3],
            Cardinal::NE => vec![1, 2, 3],
            Cardinal::W => vec![4, 5, 6],
            Cardinal::C => vec![4, 5, 6],
            Cardinal::E => vec![4, 5, 6],
            Cardinal::SW => vec![7, 8, 9],
            Cardinal::S => vec![7, 8, 9],
            Cardinal::SE => vec![7, 8, 9],
            Cardinal::Unknown => Vec::new(),
        }
    }
    /*
    get columns of a square
    */
    pub fn get_columns(self) -> Vec<u8> {
        match self {
            Cardinal::NW => vec![1, 2, 3],
            Cardinal::W => vec![1, 2, 3],
            Cardinal::SW => vec![1, 2, 3],
            Cardinal::N => vec![4, 5, 6],
            Cardinal::C => vec![4, 5, 6],
            Cardinal::S => vec![4, 5, 6],
            Cardinal::E => vec![7, 8, 9],
            Cardinal::NE => vec![7, 8, 9],
            Cardinal::SE => vec![7, 8, 9],
            Cardinal::Unknown => Vec::new(),
        }
    }
    /*
     get cells of square
    */
    pub fn _get_cells(self) -> Vec<u8> {
        match self {
            Cardinal::NW => vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
            Cardinal::N => vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
            Cardinal::NE => vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
            Cardinal::W => vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
            Cardinal::C => vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
            Cardinal::E => vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
            Cardinal::SW => vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
            Cardinal::S => vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
            Cardinal::SE => vec![60, 61, 62, 69, 70, 71, 78, 79, 80],
            Cardinal::Unknown => Vec::new(),
        }
    }
}

impl PartialEq for Cardinal {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

pub struct Accessor {
    lines: HashMap<u8, Vec<u8>>,
    columns: HashMap<u8, Vec<u8>>,
    squares: HashMap<u8, Vec<u8>>,
}

impl Accessor {
    pub fn new() -> Accessor {
        Accessor {
            lines: gen_lines(),
            columns: gen_columns(),
            squares: gen_squares(),
        }
    }

    pub fn get_line(&self, l: u8) -> Vec<u8> {
        match self.lines.get(&l) {
            None => Vec::new(),
            Some(x) => x.clone(),
        }
    }
    pub fn get_column(&self, c: u8) -> Vec<u8> {
        match self.columns.get(&c) {
            None => Vec::new(),
            Some(x) => x.clone(),
        }
    }
    pub fn get_square(&self, s: Cardinal) -> Vec<u8> {
        match self.squares.get(&(s.get_value())) {
            None => Vec::new(),
            Some(x) => x.clone(),
        }
    }
}

fn gen_squares() -> HashMap<u8, Vec<u8>> {
    let mut res = HashMap::new();
    let mut i = 1;
    for l in 0..3 {
        for c in 0..3 {
            res.insert(i, gen_square(l * 3, (l + 1) * 3, c * 3, (c + 1) * 3));
            i += 1;
        }
    }
    res
}

fn gen_square(l1: u8, l2: u8, c1: u8, c2: u8) -> Vec<u8> {
    let mut res = Vec::new();
    for line in l1..l2 {
        for column in c1..c2 {
            res.push(line * LINESIZE + column);
        }
    }
    res
}

fn gen_lines() -> HashMap<u8, Vec<u8>> {
    let mut res = HashMap::new();
    for i in 0..LINESIZE {
        res.insert(i + 1, gen_line(i));
    }
    res
}

fn gen_line(l: u8) -> Vec<u8> {
    let mut res = Vec::new();
    let mut pos = l * LINESIZE;
    for _i in 0..LINESIZE {
        res.push(pos);
        pos += 1;
    }
    res
}

fn gen_columns() -> HashMap<u8, Vec<u8>> {
    let mut res = HashMap::new();
    for i in 0..COLUMNSIZE {
        res.insert(i + 1, gen_column(i));
    }
    res
}

fn gen_column(c: u8) -> Vec<u8> {
    let mut res = Vec::new();
    let mut pos = c;
    for _i in 0..COLUMNSIZE {
        res.push(pos);
        pos += LINESIZE;
    }
    res
}

#[test]
fn lines_test() {
    assert_eq!(gen_line(0), vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));
    assert_eq!(gen_line(8), vec!(72, 73, 74, 75, 76, 77, 78, 79, 80));
    let l = gen_lines();
    assert_eq!(l.get(&1).unwrap(), &vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));
    assert_eq!(
        l.get(&9).unwrap(),
        &vec!(72, 73, 74, 75, 76, 77, 78, 79, 80)
    );
}

#[test]
fn columns_test() {
    assert_eq!(gen_column(0), vec!(0, 9, 18, 27, 36, 45, 54, 63, 72));
    assert_eq!(gen_column(8), vec!(8, 17, 26, 35, 44, 53, 62, 71, 80));
    let l = gen_columns();
    assert_eq!(l.get(&1).unwrap(), &vec!(0, 9, 18, 27, 36, 45, 54, 63, 72));
    assert_eq!(l.get(&9).unwrap(), &vec!(8, 17, 26, 35, 44, 53, 62, 71, 80));
}

#[test]
fn squares_test() {
    assert_eq!(gen_square(0, 3, 0, 3), vec!(0, 1, 2, 9, 10, 11, 18, 19, 20));
    assert_eq!(
        gen_square(3, 6, 3, 6),
        vec!(30, 31, 32, 39, 40, 41, 48, 49, 50)
    );
    assert_eq!(
        gen_square(3, 6, 0, 3),
        vec!(27, 28, 29, 36, 37, 38, 45, 46, 47)
    );
    let l = gen_squares();
    assert_eq!(l.get(&1).unwrap(), &vec!(0, 1, 2, 9, 10, 11, 18, 19, 20));
    assert_eq!(
        l.get(&4).unwrap(),
        &vec!(27, 28, 29, 36, 37, 38, 45, 46, 47)
    );
}

/*
 from a position calculate the square
*/
pub fn pos_to_square(pos: usize) -> Cardinal {
    let coord = pos_to_coord(pos);
    let res = match coord.0 {
        1..=3 => match coord.1 {
            1..=3 => 1,
            4..=6 => 2,
            7..=9 => 3,
            _ => 0,
        },
        4..=6 => match coord.1 {
            1..=3 => 4,
            4..=6 => 5,
            7..=9 => 6,
            _ => 0,
        },
        7..=9 => match coord.1 {
            1..=3 => 7,
            4..=6 => 8,
            7..=9 => 9,
            _ => 0,
        },
        _ => 0,
    };
    let tmp = Cardinal::C;
    tmp.from(res)
}
#[test]
fn pos_to_square_test() {
    //Macro (sort of)
    fn local(i: usize) -> u8 {
        pos_to_square(i).get_value()
    }
    assert_eq!(1, local(0));
    assert_eq!(1, local(1));
    assert_eq!(1, local(2));
    assert_eq!(2, local(3));
    assert_eq!(2, local(4));
    assert_eq!(2, local(5));
    assert_eq!(3, local(6));
    assert_eq!(3, local(7));
    assert_eq!(3, local(8));
    assert_eq!(1, local(9));
    assert_eq!(1, local(10));
    assert_eq!(1, local(11));
    assert_eq!(2, local(12));
    assert_eq!(2, local(13));
    assert_eq!(2, local(14));
    assert_eq!(3, local(15));
    assert_eq!(3, local(16));
    assert_eq!(3, local(17));
    assert_eq!(1, local(18));
    assert_eq!(1, local(19));
    assert_eq!(1, local(20));
    assert_eq!(2, local(21));
    assert_eq!(2, local(22));
    assert_eq!(2, local(23));
    assert_eq!(3, local(24));
    assert_eq!(3, local(25));
    assert_eq!(3, local(26));
}
/*
  from a position calculate line and column
*/
pub fn pos_to_coord(pos: usize) -> (u8, u8) {
    let pos: u8 = pos.try_into().unwrap();
    for lin in 1..=LINESIZE {
        for col in 1..=COLUMNSIZE {
            let p = col + (lin - 1) * LINESIZE - 1;
            if p == pos {
                return (lin, col);
            }
        }
    }
    panic!("Position {} not supported", pos);
}
/**
 * check the code that compute line/column from position
 **/
#[test]
fn pos_to_coord_test() {
    assert_eq!((1, 1), pos_to_coord(0));
    assert_eq!((1, 9), pos_to_coord(8));
    assert_eq!((2, 1), pos_to_coord(9));
    assert_eq!((2, 4), pos_to_coord(12));
    assert_eq!((2, 6), pos_to_coord(14));
    assert_eq!((2, 7), pos_to_coord(15));
    assert_eq!((9, 9), pos_to_coord(80));
}

pub fn coord_to_pos(line: u8, column: u8) -> usize {
    let pos = (line - 1) * LINESIZE + column - 1;
    pos.try_into().unwrap()
}
#[test]
fn coord_to_pos_test() {
    assert_eq!(0, coord_to_pos(1, 1));
    assert_eq!(8, coord_to_pos(1, 9));
    assert_eq!(9, coord_to_pos(2, 1));
    assert_eq!(12, coord_to_pos(2, 4));
    assert_eq!(14, coord_to_pos(2, 6));
    assert_eq!(15, coord_to_pos(2, 7));
    assert_eq!(80, coord_to_pos(9, 9));
}
