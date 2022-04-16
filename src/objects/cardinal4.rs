use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub enum Cardinal4 {
    _1_1,
    _1_2,
    _1_3,
    _1_4,
    _2_1,
    _2_2,
    _2_3,
    _2_4,
    _3_1,
    _3_2,
    _3_3,
    _3_4,
    _4_1,
    _4_2,
    _4_3,
    _4_4,
    Unknown,
}
impl Cardinal4 {
    pub fn get_value(self) -> u16 {
        match self {
            Cardinal4::_1_1 => 1,
            Cardinal4::_1_2 => 2,
            Cardinal4::_1_3 => 3,
            Cardinal4::_1_4 => 4,
            Cardinal4::_2_1 => 5,
            Cardinal4::_2_2 => 6,
            Cardinal4::_2_3 => 7,
            Cardinal4::_2_4 => 8,
            Cardinal4::_3_1 => 9,
            Cardinal4::_3_2 => 10,
            Cardinal4::_3_3 => 11,
            Cardinal4::_3_4 => 12,
            Cardinal4::_4_1 => 13,
            Cardinal4::_4_2 => 14,
            Cardinal4::_4_3 => 15,
            Cardinal4::_4_4 => 16,
            Cardinal4::Unknown => 0,
        }
    }

    pub fn from(self, val: u16) -> Cardinal4 {
        match val {
            1 => Cardinal4::_1_1,
            2 => Cardinal4::_1_2,
            3 => Cardinal4::_1_3,
            4 => Cardinal4::_1_4,
            5 => Cardinal4::_2_1,
            6 => Cardinal4::_2_2,
            7 => Cardinal4::_2_3,
            8 => Cardinal4::_2_4,
            9 => Cardinal4::_3_1,
            10 => Cardinal4::_3_2,
            11 => Cardinal4::_3_3,
            12 => Cardinal4::_3_4,
            13 => Cardinal4::_4_1,
            14 => Cardinal4::_4_2,
            15 => Cardinal4::_4_3,
            16 => Cardinal4::_4_4,
            _ => Cardinal4::Unknown, //default
        }
    }
    /*
     get coord of square ((line,column) ,(line,column))
    */
    pub fn get_coord(self) -> ((u16, u16), (u16, u16)) {
        match self {
            Cardinal4::_1_1 => ((1, 1), (4, 4)),
            Cardinal4::_1_2 => ((1, 5), (4, 8)),
            Cardinal4::_1_3 => ((1, 9), (4, 12)),
            Cardinal4::_1_4 => ((1, 13), (4, 16)),
            Cardinal4::_2_1 => ((5, 1), (9, 4)),
            Cardinal4::_2_2 => ((5, 5), (9, 8)),
            Cardinal4::_2_3 => ((5, 9), (8, 12)),
            Cardinal4::_2_4 => ((5, 13), (9, 16)),
            Cardinal4::_3_1 => ((10, 1), (13, 4)),
            Cardinal4::_3_2 => ((10, 5), (13, 8)),
            Cardinal4::_3_3 => ((10, 9), (13, 12)),
            Cardinal4::_3_4 => ((10, 13), (13, 16)),
            Cardinal4::_4_1 => ((14, 1), (17, 4)),
            Cardinal4::_4_2 => ((14, 5), (17, 8)),
            Cardinal4::_4_3 => ((14, 9), (17, 12)),
            Cardinal4::_4_4 => ((14, 13), (17, 16)),
            Cardinal4::Unknown => ((0, 0), (0, 0)),
        }
    }
    /*
     get other square of the same line/row
    */
    pub fn _get_other(self) -> Vec<Cardinal4> {
        match self {
            Cardinal4::_1_1 => vec![
                Cardinal4::_1_2,
                Cardinal4::_1_3,
                Cardinal4::_1_4,
                Cardinal4::_2_1,
                Cardinal4::_3_1,
                Cardinal4::_4_1,
            ],
            Cardinal4::_1_2 => vec![
                Cardinal4::_1_1,
                Cardinal4::_1_3,
                Cardinal4::_1_4,
                Cardinal4::_2_2,
                Cardinal4::_3_2,
                Cardinal4::_4_2,
            ],
            Cardinal4::_1_3 => vec![
                Cardinal4::_1_1,
                Cardinal4::_1_2,
                Cardinal4::_1_4,
                Cardinal4::_2_3,
                Cardinal4::_3_3,
                Cardinal4::_4_3,
            ],
            Cardinal4::_1_4 => vec![
                Cardinal4::_1_1,
                Cardinal4::_1_2,
                Cardinal4::_1_3,
                Cardinal4::_2_4,
                Cardinal4::_3_4,
                Cardinal4::_4_4,
            ],
            Cardinal4::_2_1 => vec![
                Cardinal4::_2_2,
                Cardinal4::_2_3,
                Cardinal4::_2_4,
                Cardinal4::_1_1,
                Cardinal4::_3_1,
                Cardinal4::_4_1,
            ],
            Cardinal4::_2_2 => vec![
                Cardinal4::_2_1,
                Cardinal4::_2_3,
                Cardinal4::_2_4,
                Cardinal4::_1_2,
                Cardinal4::_3_2,
                Cardinal4::_4_2,
            ],
            Cardinal4::_2_3 => vec![
                Cardinal4::_2_1,
                Cardinal4::_2_2,
                Cardinal4::_2_4,
                Cardinal4::_1_3,
                Cardinal4::_3_3,
                Cardinal4::_4_3,
            ],
            Cardinal4::_2_4 => vec![
                Cardinal4::_2_1,
                Cardinal4::_2_2,
                Cardinal4::_2_3,
                Cardinal4::_1_4,
                Cardinal4::_3_4,
                Cardinal4::_4_4,
            ],
            Cardinal4::_3_1 => vec![
                Cardinal4::_3_2,
                Cardinal4::_3_3,
                Cardinal4::_3_4,
                Cardinal4::_1_1,
                Cardinal4::_2_1,
                Cardinal4::_4_1,
            ],
            Cardinal4::_3_2 => vec![
                Cardinal4::_3_1,
                Cardinal4::_3_3,
                Cardinal4::_3_4,
                Cardinal4::_1_2,
                Cardinal4::_2_2,
                Cardinal4::_4_2,
            ],
            Cardinal4::_3_3 => vec![
                Cardinal4::_3_1,
                Cardinal4::_3_2,
                Cardinal4::_3_4,
                Cardinal4::_1_3,
                Cardinal4::_2_3,
                Cardinal4::_4_3,
            ],
            Cardinal4::_3_4 => vec![
                Cardinal4::_3_1,
                Cardinal4::_3_2,
                Cardinal4::_3_3,
                Cardinal4::_1_4,
                Cardinal4::_2_4,
                Cardinal4::_4_4,
            ],
            Cardinal4::_4_1 => vec![
                Cardinal4::_4_2,
                Cardinal4::_4_3,
                Cardinal4::_4_4,
                Cardinal4::_1_1,
                Cardinal4::_2_1,
                Cardinal4::_3_1,
            ],
            Cardinal4::_4_2 => vec![
                Cardinal4::_4_1,
                Cardinal4::_4_3,
                Cardinal4::_4_4,
                Cardinal4::_1_2,
                Cardinal4::_2_2,
                Cardinal4::_2_2,
            ],
            Cardinal4::_4_3 => vec![
                Cardinal4::_4_1,
                Cardinal4::_4_2,
                Cardinal4::_4_4,
                Cardinal4::_1_3,
                Cardinal4::_3_3,
                Cardinal4::_3_3,
            ],
            Cardinal4::_4_4 => vec![
                Cardinal4::_4_1,
                Cardinal4::_4_2,
                Cardinal4::_4_3,
                Cardinal4::_1_4,
                Cardinal4::_2_4,
                Cardinal4::_2_4,
            ],
            Cardinal4::Unknown => Vec::new(),
        }
    }
    /*
    get all squares
    */
    pub fn get_all(self) -> Vec<Cardinal4> {
        vec![
            Cardinal4::_1_1,
            Cardinal4::_1_2,
            Cardinal4::_1_3,
            Cardinal4::_1_4,
            Cardinal4::_2_1,
            Cardinal4::_2_2,
            Cardinal4::_2_3,
            Cardinal4::_2_4,
            Cardinal4::_3_1,
            Cardinal4::_3_2,
            Cardinal4::_3_3,
            Cardinal4::_3_4,
            Cardinal4::_4_1,
            Cardinal4::_4_2,
            Cardinal4::_4_3,
            Cardinal4::_4_4,
        ]
    }
    /*
     get lines of a square
    */
    pub fn get_lines(self) -> Vec<u16> {
        match self {
            Cardinal4::_1_1 => vec![1, 2, 3, 4],
            Cardinal4::_1_2 => vec![1, 2, 3, 4],
            Cardinal4::_1_3 => vec![1, 2, 3, 4],
            Cardinal4::_1_4 => vec![1, 2, 3, 4],
            Cardinal4::_2_1 => vec![5, 6, 7, 8],
            Cardinal4::_2_2 => vec![5, 6, 7, 8],
            Cardinal4::_2_3 => vec![5, 6, 7, 8],
            Cardinal4::_2_4 => vec![5, 6, 7, 8],
            Cardinal4::_3_1 => vec![9, 10, 11, 12],
            Cardinal4::_3_2 => vec![9, 10, 11, 12],
            Cardinal4::_3_3 => vec![9, 10, 11, 12],
            Cardinal4::_3_4 => vec![9, 10, 11, 12],
            Cardinal4::_4_1 => vec![13, 14, 15, 16],
            Cardinal4::_4_2 => vec![13, 14, 15, 16],
            Cardinal4::_4_3 => vec![13, 14, 15, 16],
            Cardinal4::_4_4 => vec![13, 14, 15, 16],
            Cardinal4::Unknown => Vec::new(),
        }
    }
    /*
    get columns of a square
    */
    pub fn get_columns(self) -> Vec<u16> {
        match self {
            Cardinal4::_1_1 => vec![1, 2, 3, 4],
            Cardinal4::_2_1 => vec![1, 2, 3, 4],
            Cardinal4::_3_1 => vec![1, 2, 3, 4],
            Cardinal4::_4_1 => vec![1, 2, 3, 4],
            Cardinal4::_1_2 => vec![5, 6, 7, 8],
            Cardinal4::_2_2 => vec![5, 6, 7, 8],
            Cardinal4::_3_2 => vec![5, 6, 7, 8],
            Cardinal4::_4_2 => vec![5, 6, 7, 8],
            Cardinal4::_1_3 => vec![9, 10, 11, 12],
            Cardinal4::_2_3 => vec![9, 10, 11, 12],
            Cardinal4::_3_3 => vec![9, 10, 11, 12],
            Cardinal4::_4_3 => vec![9, 10, 11, 12],
            Cardinal4::_1_4 => vec![13, 14, 15, 16],
            Cardinal4::_2_4 => vec![13, 14, 15, 16],
            Cardinal4::_3_4 => vec![13, 14, 15, 16],
            Cardinal4::_4_4 => vec![13, 14, 15, 16],
            Cardinal4::Unknown => Vec::new(),
        }
    }
    /*
     get cells of square
    */
    pub fn _get_cells(self) -> Vec<u16> {
        match self {
            Cardinal4::_1_1 => vec![0, 1, 2, 3, 16, 17, 18, 19, 32, 33, 34, 35],
            Cardinal4::_1_2 => vec![4, 5, 6, 7, 20, 21, 22, 23, 36, 37, 38, 39],
            Cardinal4::_1_3 => vec![8, 9, 10, 11, 24, 25, 26, 27, 40, 41, 42, 43],
            Cardinal4::_1_4 => vec![12, 13, 14, 15, 28, 29, 30, 31, 44, 45, 46, 47],
            Cardinal4::_2_1 => vec![49, 49, 50, 51, 64, 65, 66, 67, 80, 81, 82, 83],
            Cardinal4::_2_2 => vec![52, 53, 54, 55, 68, 69, 70, 71, 84, 85, 86, 87],
            Cardinal4::_2_3 => vec![56, 57, 58, 59, 72, 73, 74, 75, 88, 89, 90, 91],
            Cardinal4::_2_4 => vec![60, 61, 62, 63, 76, 77, 78, 79, 92, 93, 94, 95],
            Cardinal4::_3_1 => vec![96, 97, 98, 99, 112, 113, 114, 115, 128, 129, 130, 131],
            Cardinal4::_3_2 => vec![100, 101, 102, 103, 116, 117, 118, 119, 132, 133, 134, 135],
            Cardinal4::_3_3 => vec![104, 105, 106, 107, 120, 121, 122, 123, 136, 137, 138, 139],
            Cardinal4::_3_4 => vec![108, 109, 110, 111, 124, 125, 126, 127, 140, 141, 142, 143],
            Cardinal4::_4_1 => vec![144, 145, 146, 147, 160, 161, 162, 163, 176, 177, 178, 179],
            Cardinal4::_4_2 => vec![148, 149, 150, 151, 164, 165, 166, 167, 180, 181, 182, 183],
            Cardinal4::_4_3 => vec![152, 153, 154, 155, 168, 169, 170, 171, 184, 185, 186, 187],
            Cardinal4::_4_4 => vec![156, 157, 158, 159, 172, 173, 174, 175, 188, 189, 191, 495],
            Cardinal4::Unknown => Vec::new(),
        }
    }
}

impl PartialEq for Cardinal4 {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
