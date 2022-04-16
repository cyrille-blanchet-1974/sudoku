use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub enum Cardinal2 {
    _1_1,
    _1_2,
    _2_1,
    _2_2,
    Unknown,
}
impl Cardinal2 {
    pub fn get_value(self) -> u16 {
        match self {
            Cardinal2::_1_1 => 1,
            Cardinal2::_1_2 => 2,
            Cardinal2::_2_1 => 3,
            Cardinal2::_2_2 => 4,
            Cardinal2::Unknown => 0,
        }
    }

    pub fn from(self, val: u16) -> Cardinal2 {
        match val {
            1 => Cardinal2::_1_1,
            2 => Cardinal2::_1_2,
            3 => Cardinal2::_2_1,
            4 => Cardinal2::_2_2,
            _ => Cardinal2::Unknown, //default
        }
    }
    /*
     get coord of square ((line,column) ,(line,column))
    */
    pub fn get_coord(self) -> ((u16, u16), (u16, u16)) {
        match self {
            Cardinal2::_1_1 => ((1, 1), (2, 2)),
            Cardinal2::_1_2 => ((1, 3), (2, 4)),
            Cardinal2::_2_1 => ((3, 1), (4, 2)),
            Cardinal2::_2_2 => ((3, 3), (4, 4)),
            Cardinal2::Unknown => ((0, 0), (0, 0)),
        }
    }
    /*
     get other square of the same line/row
    */
    pub fn _get_other(self) -> Vec<Cardinal2> {
        match self {
            Cardinal2::_1_1 => vec![Cardinal2::_1_2, Cardinal2::_2_1],
            Cardinal2::_1_2 => vec![Cardinal2::_1_1, Cardinal2::_2_2],
            Cardinal2::_2_1 => vec![Cardinal2::_1_1, Cardinal2::_2_2],
            Cardinal2::_2_2 => vec![Cardinal2::_1_2, Cardinal2::_2_1],
            Cardinal2::Unknown => Vec::new(),
        }
    }
    /*
    get all squares
    */
    pub fn get_all(self) -> Vec<Cardinal2> {
        vec![
            Cardinal2::_1_1,
            Cardinal2::_1_2,
            Cardinal2::_2_1,
            Cardinal2::_2_2,
        ]
    }
    /*
     get lines of a square
    */
    pub fn get_lines(self) -> Vec<u16> {
        match self {
            Cardinal2::_1_1 => vec![1, 2],
            Cardinal2::_1_2 => vec![1, 2],
            Cardinal2::_2_1 => vec![3, 4],
            Cardinal2::_2_2 => vec![3, 4],
            Cardinal2::Unknown => Vec::new(),
        }
    }
    /*
    get columns of a square
    */
    pub fn get_columns(self) -> Vec<u16> {
        match self {
            Cardinal2::_1_1 => vec![1, 2],
            Cardinal2::_2_1 => vec![1, 2],
            Cardinal2::_1_2 => vec![3, 4],
            Cardinal2::_2_2 => vec![3, 4],
            Cardinal2::Unknown => Vec::new(),
        }
    }
    /*
     get cells of square
    */
    pub fn _get_cells(self) -> Vec<u16> {
        match self {
            Cardinal2::_1_1 => vec![0, 1, 4, 5],
            Cardinal2::_1_2 => vec![2, 3, 6, 7],
            Cardinal2::_2_1 => vec![8, 9, 12, 13],
            Cardinal2::_2_2 => vec![10, 11, 14, 15],
            Cardinal2::Unknown => Vec::new(),
        }
    }
}

impl PartialEq for Cardinal2 {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
