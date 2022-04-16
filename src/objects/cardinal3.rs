use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub enum Cardinal3 {
    _1_1,
    _1_2,
    _1_3,
    _2_1,
    _2_2,
    _2_3,
    _3_1,
    _3_2,
    _3_3,

    Unknown,
}
impl Cardinal3 {
    pub fn get_value(self) -> u16 {
        match self {
            Cardinal3::_1_1 => 1,
            Cardinal3::_1_2 => 2,
            Cardinal3::_1_3 => 3,
            Cardinal3::_2_1 => 4,
            Cardinal3::_2_2 => 5,
            Cardinal3::_2_3 => 6,
            Cardinal3::_3_1 => 7,
            Cardinal3::_3_2 => 8,
            Cardinal3::_3_3 => 9,
            Cardinal3::Unknown => 0,
        }
    }

    pub fn from(self, val: u16) -> Cardinal3 {
        match val {
            1 => Cardinal3::_1_1,
            2 => Cardinal3::_1_2,
            3 => Cardinal3::_1_3,
            4 => Cardinal3::_2_1,
            5 => Cardinal3::_2_2,
            6 => Cardinal3::_2_3,
            7 => Cardinal3::_3_1,
            8 => Cardinal3::_3_2,
            9 => Cardinal3::_3_3,
            _ => Cardinal3::Unknown, //default
        }
    }
    /*
     get coord of square ((line,column) ,(line,column))
    */
    pub fn get_coord(self) -> ((u16, u16), (u16, u16)) {
        match self {
            Cardinal3::_1_1 => ((1, 1), (3, 3)),
            Cardinal3::_1_2 => ((1, 4), (3, 6)),
            Cardinal3::_1_3 => ((1, 7), (3, 9)),
            Cardinal3::_2_1 => ((4, 1), (6, 3)),
            Cardinal3::_2_2 => ((4, 4), (6, 6)),
            Cardinal3::_2_3 => ((4, 7), (6, 9)),
            Cardinal3::_3_1 => ((7, 1), (9, 3)),
            Cardinal3::_3_2 => ((7, 7), (9, 6)),
            Cardinal3::_3_3 => ((7, 7), (9, 9)),
            Cardinal3::Unknown => ((0, 0), (0, 0)),
        }
    }
    /*
     get other square of the same line/row
    */
    pub fn _get_other(self) -> Vec<Cardinal3> {
        match self {
            Cardinal3::_1_1 => vec![
                Cardinal3::_1_2,
                Cardinal3::_1_3,
                Cardinal3::_2_1,
                Cardinal3::_3_1,
            ],
            Cardinal3::_1_2 => vec![
                Cardinal3::_1_1,
                Cardinal3::_1_3,
                Cardinal3::_2_2,
                Cardinal3::_3_2,
            ],
            Cardinal3::_1_3 => vec![
                Cardinal3::_1_1,
                Cardinal3::_1_2,
                Cardinal3::_2_3,
                Cardinal3::_3_3,
            ],
            Cardinal3::_2_1 => vec![
                Cardinal3::_1_1,
                Cardinal3::_3_1,
                Cardinal3::_2_2,
                Cardinal3::_2_3,
            ],
            Cardinal3::_2_2 => vec![
                Cardinal3::_1_2,
                Cardinal3::_3_2,
                Cardinal3::_2_1,
                Cardinal3::_2_3,
            ],
            Cardinal3::_2_3 => vec![
                Cardinal3::_1_3,
                Cardinal3::_3_3,
                Cardinal3::_2_2,
                Cardinal3::_2_1,
            ],
            Cardinal3::_3_1 => vec![
                Cardinal3::_1_1,
                Cardinal3::_2_1,
                Cardinal3::_3_2,
                Cardinal3::_3_3,
            ],
            Cardinal3::_3_2 => vec![
                Cardinal3::_3_1,
                Cardinal3::_3_3,
                Cardinal3::_2_2,
                Cardinal3::_1_2,
            ],
            Cardinal3::_3_3 => vec![
                Cardinal3::_3_1,
                Cardinal3::_3_2,
                Cardinal3::_2_3,
                Cardinal3::_1_3,
            ],
            Cardinal3::Unknown => Vec::new(),
        }
    }
    /*
    get all squares
    */
    pub fn get_all(self) -> Vec<Cardinal3> {
        vec![
            Cardinal3::_1_1,
            Cardinal3::_1_2,
            Cardinal3::_1_3,
            Cardinal3::_2_1,
            Cardinal3::_2_2,
            Cardinal3::_2_3,
            Cardinal3::_3_1,
            Cardinal3::_3_2,
            Cardinal3::_3_3,
        ]
    }
    /*
     get lines of a square
    */
    pub fn get_lines(self) -> Vec<u16> {
        match self {
            Cardinal3::_1_1 => vec![1, 2, 3],
            Cardinal3::_1_2 => vec![1, 2, 3],
            Cardinal3::_1_3 => vec![1, 2, 3],
            Cardinal3::_2_1 => vec![4, 5, 6],
            Cardinal3::_2_2 => vec![4, 5, 6],
            Cardinal3::_2_3 => vec![4, 5, 6],
            Cardinal3::_3_1 => vec![7, 8, 9],
            Cardinal3::_3_2 => vec![7, 8, 9],
            Cardinal3::_3_3 => vec![7, 8, 9],
            Cardinal3::Unknown => Vec::new(),
        }
    }
    /*
    get columns of a square
    */
    pub fn get_columns(self) -> Vec<u16> {
        match self {
            Cardinal3::_1_1 => vec![1, 2, 3],
            Cardinal3::_2_1 => vec![1, 2, 3],
            Cardinal3::_3_1 => vec![1, 2, 3],
            Cardinal3::_1_2 => vec![4, 5, 6],
            Cardinal3::_2_2 => vec![4, 5, 6],
            Cardinal3::_3_2 => vec![4, 5, 6],
            Cardinal3::_2_3 => vec![7, 8, 9],
            Cardinal3::_1_3 => vec![7, 8, 9],
            Cardinal3::_3_3 => vec![7, 8, 9],
            Cardinal3::Unknown => Vec::new(),
        }
    }
    /*
     get cells of square
    */
    pub fn _get_cells(self) -> Vec<u16> {
        match self {
            Cardinal3::_1_1 => vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
            Cardinal3::_1_2 => vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
            Cardinal3::_1_3 => vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
            Cardinal3::_2_1 => vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
            Cardinal3::_2_2 => vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
            Cardinal3::_2_3 => vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
            Cardinal3::_3_1 => vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
            Cardinal3::_3_2 => vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
            Cardinal3::_3_3 => vec![60, 61, 62, 69, 70, 71, 78, 79, 80],
            Cardinal3::Unknown => Vec::new(),
        }
    }
}

impl PartialEq for Cardinal3 {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
