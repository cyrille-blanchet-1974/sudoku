use super::cell::*;
use super::constant::*;
use super::grid::*;
use super::read::from_vecvec;
use std::convert::TryInto;

/*
 here we use cpu power to try all possibilities

*/
pub struct ResolverForce {
    debug: bool,
}

impl ResolverForce {
    pub fn new(debug: bool) -> ResolverForce {
        ResolverForce { debug }
    }

    //display the grid
    pub fn display(&mut self, v: &[Vec<u8>]) {
        let mut g = from_vecvec(v, self.debug);
        g.display();
    }

    pub fn grid_to_vec(&mut self, g: &mut Grid) -> Vec<Vec<u8>> {
        let mut flat = Vec::new();
        for l in 0..MAX {
            let mut line = Vec::new();
            for c in 0..MAX {
                let pos: usize = (l * MAX + c).try_into().unwrap();
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

    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        let mut flat = self.grid_to_vec(g);
        if self.debug {
            self.display(&flat);
        }
        let res = self.raw_force(&mut flat);
        if res {
            println!("resolution ok");
            self.display(&flat);
        }
        res
    }

    pub fn count_solutions(&mut self, g: &mut Grid) -> u8 {
        let mut flat = self.grid_to_vec(g);
        if self.debug {
            self.display(&flat);
        }
        let mut nb = 0;
        self.count_sol(&mut flat, &mut nb);
        nb
    }

    pub fn in_row(&mut self, g: &[Vec<u8>], row: usize, val: u8) -> bool {
        for col in 0..MAX {
            let c: usize = col.try_into().unwrap();
            if g[row][c] == val {
                return true;
            }
        }
        false
    }

    pub fn in_col(&mut self, g: &[Vec<u8>], col: usize, val: u8) -> bool {
        for row in 0..MAX {
            let r: usize = row.try_into().unwrap();
            if g[r][col] == val {
                return true;
            }
        }
        false
    }

    pub fn in_square(&mut self, g: &[Vec<u8>], row: usize, col: usize, val: u8) -> bool {
        let lrow = row - row % 3;
        let lcol = col - col % 3;
        for r in g.iter().skip(lrow).take(3) {
            for c in r.iter().skip(lcol).take(3) {
                if *c == val {
                    return true;
                }
            }
        }
        false
    }

    pub fn valid(&mut self, g: &[Vec<u8>], row: usize, col: usize, val: u8) -> bool {
        !self.in_col(g, col, val) && !self.in_row(g, row, val) && !self.in_square(g, row, col, val)
    }

    pub fn raw_force(&mut self, g: &mut Vec<Vec<u8>>) -> bool {
        for row in 0..MAX {
            let r: usize = row.try_into().unwrap();
            for col in 0..MAX {
                let c: usize = col.try_into().unwrap();
                if g[r][c] == 0 {
                    for val in 1..=MAX {
                        if self.valid(g, r, c, val) {
                            //let mut cop = self.copy(g);
                            //cop[r][c] = val;
                            g[r][c] = val;
                            let res = self.raw_force(g);
                            if res {
                                return true;
                            } else {
                                g[r][c] = 0;
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    pub fn count_sol(&mut self, g: &mut Vec<Vec<u8>>, nb_solutions: &mut u8) -> bool {
        for row in 0..MAX {
            let r: usize = row.try_into().unwrap();
            for col in 0..MAX {
                let c: usize = col.try_into().unwrap();
                if g[r][c] == 0 {
                    for val in 1..=MAX {
                        if self.valid(g, r, c, val) {
                            //let mut cop = self.copy(g);
                            //cop[r][c] = val;
                            g[r][c] = val;
                            //if self.count_sol(&mut cop, nb_solutions) {
                            if self.count_sol(g, nb_solutions) {
                                //g[r][c] = 0;
                                if self.debug {
                                    println!("{}/{}={} ({})", r, c, val, nb_solutions);
                                }
                                *nb_solutions +=1;
                                //return true;                                
                            } else {
                                //g[r][c] = 0;
                                //return false;
                            }
                            g[r][c] = 0;
                        }
                    }
                }
            }
        }
        //*nb_solutions +=1;
        true
    }

    pub fn copy(&mut self, g: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut second = Vec::new();
        for row in 0..MAX {
            let r: usize = row.try_into().unwrap();
            let mut line = Vec::new();
            for col in 0..MAX {
                let c: usize = col.try_into().unwrap();
                line.push(g[r][c]);
            }
            second.push(line);
        }
        second
    }
}
