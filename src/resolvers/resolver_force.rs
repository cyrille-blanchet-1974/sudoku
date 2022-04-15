use super::super::objects::cell::*;
use super::super::objects::grid::*;
use super::super::objects::metrics::*;
use super::super::read::from_vecvec;
use std::convert::TryInto;

/*
 here we use cpu power to found a solution
*/
pub struct ResolverForce {
    debug: bool,
    data: Vec<Vec<u8>>,
    metrics: Metrics,
}

impl ResolverForce {
    pub fn new(debug: bool, g: &mut Grid) -> ResolverForce {
        let flat = grid_to_vec(g);
        ResolverForce {
            debug,
            data: flat,
            metrics: g.get_metrics(),
        }
    }

    //display the grid
    pub fn display(&mut self) {
        let mut g = from_vecvec(self.metrics.get_square_side(), &self.data, self.debug);
        g.display_bw();
    }

    pub fn resolve(&mut self) -> bool {
        let res = self.raw_force();
        if res {
            println!("resolution ok");
            self.display();
        }
        res
    }

    pub fn in_row(&mut self, row: usize, val: u8) -> bool {
        for col in 0..self.metrics.get_max() {
            let c: usize = col.try_into().unwrap();
            if self.data[row][c] == val {
                return true;
            }
        }
        false
    }

    pub fn in_col(&mut self, col: usize, val: u8) -> bool {
        for row in 0..self.metrics.get_max() {
            let r: usize = row.try_into().unwrap();
            if self.data[r][col] == val {
                return true;
            }
        }
        false
    }

    pub fn in_square(&mut self, row: usize, col: usize, val: u8) -> bool {
        let side: usize = self.metrics.get_square_side().try_into().unwrap();
        let lrow = row - row % side;
        let lcol = col - col % side;
        for r in self.data.iter().skip(lrow).take(side) {
            for c in r.iter().skip(lcol).take(side) {
                if *c == val {
                    return true;
                }
            }
        }
        false
    }

    pub fn valid(&mut self, row: usize, col: usize, val: u8) -> bool {
        !self.in_col(col, val) && !self.in_row(row, val) && !self.in_square(row, col, val)
    }

    pub fn raw_force(&mut self) -> bool {
        for row in 0..self.metrics.get_max() {
            let r: usize = row.try_into().unwrap();
            for col in 0..self.metrics.get_max() {
                let c: usize = col.try_into().unwrap();
                if self.data[r][c] == 0 {
                    for val in 1..=self.metrics.get_max() {
                        if self.valid(r, c, val) {
                            self.data[r][c] = val;
                            let res = self.raw_force();
                            if res {
                                return true;
                            } else {
                                self.data[r][c] = 0;
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}

pub fn grid_to_vec(g: &mut Grid) -> Vec<Vec<u8>> {
    let max = g.get_metrics().get_max();
    let mut flat = Vec::new();
    for l in 0..max {
        let mut line = Vec::new();
        for c in 0..max {
            let pos: usize = (l * max + c).try_into().unwrap();
            let cell: &mut Cell = g.get_cell(pos);
            if cell.is_resolved() {
                line.push(cell.get_answer().unwrap());
            } else {
                line.push(0);
            }
        }
        flat.push(line);
    }
    flat
}
