use std::cmp::PartialEq;

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
    pub fn get_value(self) -> u16 {
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

    pub fn from(self, val: u16) -> Cardinal {
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
    pub fn get_coord(self) -> ((u16, u16), (u16, u16)) {
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
    pub fn get_lines(self) -> Vec<u16> {
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
    pub fn get_columns(self) -> Vec<u16> {
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
    pub fn _get_cells(self) -> Vec<u16> {
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
