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
        for lin in 1..=self.metrics.get_nb_line() {
            for col in 1..=self.metrics.get_nb_column() {
                let p = col + (lin - 1) * self.metrics.get_nb_line() - 1;
                if p == pos {
                    return (lin, col);
                }
            }
        }
        panic!("Position {} not supported", pos);
    }

    pub fn coord_to_pos(&self, line: u16, column: u16) -> usize {
        let pos = (line - 1) * self.metrics.get_nb_line() + column - 1;
        pos.into()
    }

    /*
    from a position calculate the square
    */
    pub fn pos_to_square(&self, pos: usize) -> Cardinal {
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
        let tmp = Cardinal::C;
        tmp.from(res)
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
        a.pos_to_square(i).get_value()
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
