use super::cardinal::*;
use super::metrics::*;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub struct CoordConverter {
    metrics: Metrics,
}

impl CoordConverter {
    pub fn new(squareside: u16) -> CoordConverter {
        CoordConverter {
            metrics: Metrics::new(squareside),
        }
    }

    /*
    from a position calculate line and column
    */
    pub fn pos_to_coord(&self, pos: usize) -> (u16, u16) {
        let pos: u16 = pos.try_into().unwrap();
        let line = pos / self.metrics.get_nb_column();
        let column = pos % self.metrics.get_nb_column();
        if line > self.metrics.get_nb_line() || column > self.metrics.get_nb_column() {
            panic!("Position {} not supported", pos);
        }
        (line + 1, column + 1)
    }
    pub fn coord_to_pos(&self, line: u16, column: u16) -> usize {
        let pos = (line - 1) * self.metrics.get_nb_line() + column - 1;
        pos.into()
    }

    /*
    from a position calculate the square
    */
    pub fn pos_to_square(&self, pos: usize) -> Cardinal {
        match self.metrics.get_square_side() {
            2 => self.pos_to_square2(pos),
            3 => self.pos_to_square3(pos),
            4 => self.pos_to_square4(pos),
            _ => {
                let tmp = Cardinal::_1_1;
                tmp.from(0, self.metrics.get_square_side())
            }
        }
    }

    fn pos_to_square3(&self, pos: usize) -> Cardinal {
        let coord = self.pos_to_coord(pos);
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
        let tmp = Cardinal::_1_1;
        tmp.from(res, self.metrics.get_square_side())
    }

    fn pos_to_square2(&self, pos: usize) -> Cardinal {
        let coord = self.pos_to_coord(pos);
        let res = match coord.0 {
            1..=2 => match coord.1 {
                1..=2 => 1,
                3..=4 => 2,
                _ => 0,
            },
            3..=4 => match coord.1 {
                1..=2 => 3,
                3..=4 => 4,
                _ => 0,
            },
            _ => 0,
        };
        let tmp = Cardinal::_1_1;
        tmp.from(res, self.metrics.get_square_side())
    }

    fn pos_to_square4(&self, pos: usize) -> Cardinal {
        let coord = self.pos_to_coord(pos);
        let res = match coord.0 {
            1..=4 => match coord.1 {
                1..=4 => 1,
                5..=8 => 2,
                9..=12 => 3,
                13..=16 => 4,
                _ => 0,
            },
            5..=8 => match coord.1 {
                1..=4 => 5,
                5..=8 => 6,
                9..=12 => 7,
                13..=16 => 8,
                _ => 0,
            },
            9..=12 => match coord.1 {
                1..=4 => 9,
                5..=8 => 10,
                9..=12 => 11,
                13..=16 => 12,
                _ => 0,
            },
            13..=16 => match coord.1 {
                1..=4 => 13,
                5..=8 => 14,
                9..=12 => 15,
                13..=16 => 16,
                _ => 0,
            },
            _ => 0,
        };
        let tmp = Cardinal::_1_1;
        tmp.from(res, self.metrics.get_square_side())
    }
}

/**
 * check the code that compute line/column from position
 **/
#[test]
fn pos_to_coord_test() {
    let a = CoordConverter::new(3);
    assert_eq!((1, 1), a.pos_to_coord(0));
    assert_eq!((1, 9), a.pos_to_coord(8));
    assert_eq!((2, 1), a.pos_to_coord(9));
    assert_eq!((2, 4), a.pos_to_coord(12));
    assert_eq!((2, 6), a.pos_to_coord(14));
    assert_eq!((2, 7), a.pos_to_coord(15));
    assert_eq!((9, 9), a.pos_to_coord(80));
}

#[test]
fn coord_to_pos_test() {
    let a = CoordConverter::new(3);
    assert_eq!(0, a.coord_to_pos(1, 1));
    assert_eq!(8, a.coord_to_pos(1, 9));
    assert_eq!(9, a.coord_to_pos(2, 1));
    assert_eq!(12, a.coord_to_pos(2, 4));
    assert_eq!(14, a.coord_to_pos(2, 6));
    assert_eq!(15, a.coord_to_pos(2, 7));
    assert_eq!(80, a.coord_to_pos(9, 9));
}

#[test]
fn pos_to_square_test() {
    //Macro (sort of)
    fn local(i: usize) -> u16 {
        let a = CoordConverter::new(3);
        a.pos_to_square(i).get_value(3)
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
