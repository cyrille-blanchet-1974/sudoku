use super::constant::*;
use super::grid::*;
use super::read::from_vecvec;
use super::resolver_force::grid_to_vec;
use std::convert::TryInto;
/*
 here we use cpu power to try all possibilities

*/
pub struct ResolverCount {
    debug: bool,
    data: Vec<Vec<u8>>,
}

impl ResolverCount {
    pub fn new(debug: bool, g: &mut Grid) -> ResolverCount {
        let flat = grid_to_vec(g);
        ResolverCount { debug, data: flat }
    }

    //display the grid
    pub fn display(&mut self) {
        let mut g = from_vecvec(&self.data, self.debug);
        g.display_bw();
    }

    pub fn count_solutions(&mut self) -> u8 {
        let mut nb = 0;
        self.count_sol(&mut nb, 0);
        nb
    }

    pub fn in_row(&mut self, row: usize, val: u8) -> bool {
        for col in 0..MAX {
            let c: usize = col.try_into().unwrap();
            if self.data[row][c] == val {
                return true;
            }
        }
        false
    }

    pub fn in_col(&mut self, col: usize, val: u8) -> bool {
        for row in 0..MAX {
            let r: usize = row.try_into().unwrap();
            if self.data[r][col] == val {
                return true;
            }
        }
        false
    }

    pub fn in_square(&mut self, row: usize, col: usize, val: u8) -> bool {
        let lrow = row - row % 3;
        let lcol = col - col % 3;
        for r in self.data.iter().skip(lrow).take(3) {
            for c in r.iter().skip(lcol).take(3) {
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

    pub fn count_sol(&mut self, nb_solutions: &mut u8, depth: u8) -> bool {
        println!("depth {}", depth);
        self.display();
        for row in 0..MAX {
            let r: usize = row.try_into().unwrap();
            for col in 0..MAX {
                let c: usize = col.try_into().unwrap();
                if self.data[r][c] == 0 {
                    let mut found_one_ok = false;
                    for val in 1..=MAX {
                        if self.valid(r, c, val) {
                            found_one_ok = true;
                            println!("{} try {} on {}/{} ", depth, val, r, c);
                            //put these value
                            self.data[r][c] = val;
                            //test resolution with it
                            if self.count_sol(nb_solutions, depth + 1) {
                                //if solution is working
                                if self.debug {
                                    println!("{}/{}={} ({})", r, c, val, nb_solutions);
                                }
                                //display these solution
                                self.display();
                                //count it
                                *nb_solutions += 1;
                                //return true;
                            } else {
                                //if not working
                                //return false;
                                println!("{} reset {}/{}", depth, r, c);
                                self.data[r][c] = 0;
                                return false;
                            }
                            //whateve the solution work or not
                            //reset and try other solutions
                            //println!("reset {}/{}", r, c);
                            //self.data[r][c] = 0;
                        }
                    }
                    if !found_one_ok {
                        //if no value is correct for these position stop resolving
                        //and callback
                        println!("{} no results for {}/{}", depth, r, c);
                        self.data[r][c] = 0;
                        return false; //impossible grid
                    }
                }
            }
        }
        //if we get here,
        //display these solution
        self.display();
        //count it
        *nb_solutions += 1;
        true
    }
}
