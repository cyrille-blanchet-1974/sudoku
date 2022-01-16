use super::cell::*;
use super::grid::*;
use super::resolver_lvl1::*;
use super::resolver_lvl2::*;
use super::resolver_lvl3::*;
use super::resolver_lvl4::*;

pub struct Resolver {
    step: u32,
    nblvl1: u32,
    nblvl1ko: u32,
    nblvl2: u32,
    nblvl2ko: u32,
    nblvl3: u32,
    nblvl3ko: u32,
    nblvl4: u32,
    nblvl4ko: u32,
    nblvl9guess: u32,
    nblvl9wrongguess: u32,
    resolver1: ResolverLvl1,
    resolver2: ResolverLvl2,
    resolver3: ResolverLvl3,
    resolver4: ResolverLvl4,
    debug: bool,
    display: bool,
}

impl Resolver {
    pub fn new(debug: bool, display: bool) -> Resolver {
        Resolver {
            step: 1,
            nblvl1: 0,
            nblvl1ko: 0,
            nblvl2: 0,
            nblvl2ko: 0,
            nblvl3: 0,
            nblvl3ko: 0,
            nblvl4: 0,
            nblvl4ko: 0,
            nblvl9guess: 0,
            nblvl9wrongguess: 0,
            resolver1: ResolverLvl1::new(),
            resolver2: ResolverLvl2::new(),
            resolver3: ResolverLvl3::new(),
            resolver4: ResolverLvl4::new(),
            debug,
            display,
        }
    }

    pub fn display_stats(&mut self) {
        println!(
            "Called {} times level 1 ({} times with no new result)",
            self.nblvl1, self.nblvl1ko
        );
        println!(
            "Called {} times level 2 ({} times with no new result)",
            self.nblvl2, self.nblvl2ko
        );
        println!(
            "Called {} times level 3 ({} times with no new result)",
            self.nblvl3, self.nblvl3ko
        );
        println!(
            "Called {} times level 4 ({} times with no new result)",
            self.nblvl4, self.nblvl4ko
        );
        println!(
            "Made {} guess at level 4, {} of those were wrong",
            self.nblvl9guess, self.nblvl9wrongguess
        );
    }

    pub fn go(&mut self, g: &mut Grid) -> bool {
        self.gos(g, "".to_string())
    }
    pub fn gos(&mut self, g: &mut Grid, spacer: String) -> bool {
        if self.display {
            g.display();
        }
        //if already resolved...
        if !g.is_resolved() {
            //loop until no more to solve
            loop {
                if self.debug {
                    println!("--------------------step {}--------------------", self.step);
                }
                if !self.resolve(g, "".to_string()) {
                    break;
                }
                if self.display {
                    g.display();
                }
                if self.debug {
                    g.display_lefts();
                }
                if !g.is_valid() {
                    if self.debug {
                        println!("Error in the grid!");
                    }
                    return false;
                }
            }
        }
        let res = g.is_resolved();
        if !res {
            return self.resolve_lvl9(g, spacer);
        }
        if self.display {
            g.display();
        }
        if !res {
            g.debug();
        }
        res
    }

    fn resolve_lvl9(&mut self, g: &mut Grid, spacer: String) -> bool {
        //let run level 9 -> guesses
        //first made a copy of
        let mut sav = g.clone();
        let sav_step = self.step;
        self.nblvl9guess += 1;
        //second find a unsolved cell
        match sav.get_a_guess() {
            None => {
                println!("Strange error: grid not solved by with no unsolved cell (or unsoved cell with no possibles values)");
                false
            }
            Some(val) => {
                let cell: &mut Cell = g.get_cell(val);
                let guess = sav.less_used(cell.get_possibles());
                let line = cell.get_line();
                let column = cell.get_column();
                //on the saved grid, let try this value on the cell
                if self.debug {
                    println!(
                        "{}Lvl9-> try value {} on cell l:{}/c:{}",
                        spacer, guess, line, column
                    );
                }
                let mut s = String::new();
                s.push_str(&spacer);
                s.push_str("___");
                /*if display && debug {
                    g.debug();
                }
                if display && debug {
                    sav.debug();
                }*/
                sav.set_val(line, column, guess, CellType::Guess);
                if self.gos(&mut sav, s.clone()) {
                    //we found the solution
                    g.copy_from(sav);
                    true
                } else {
                    if self.debug {
                        println!(
                            "{}Lvl9-> wrong guess so value {} is not possible for on cell l:{}/c:{} -> restoring previous grid (from step {})",
                            spacer,guess, line, column,sav_step
                        );
                    }
                    self.nblvl9wrongguess += 1;
                    //wrong guess -> remove this value from the possibles of the original grid and continue
                    g.remove_a_possible(line, column, guess);
                    self.gos(g, s)
                }
            }
        }
    }

    pub fn resolve(&mut self, g: &mut Grid, space: String) -> bool {
        self.step += 1;
        //try 3 first type of resolution
        //only solution when removing founds of the same line columnand square
        let res1 = self.resolver1.resolve(g);
        self.nblvl1 += 1;
        if !res1 {
            self.nblvl1ko += 1;
        } else if self.debug {
            let trc = self.resolver1.get_trace();
            println!("{}Level1 => {}", space, trc);
        }
        //value fount in two other line and two other column of a square
        let res2 = self.resolver2.resolve(g);
        self.nblvl2 += 1;
        if !res2 {
            self.nblvl2ko += 1;
        } else if self.debug {
            let trc = self.resolver2.get_trace();
            println!("{}Level2 => {}", space, trc);
        }
        //value not in the possible of other cells of the same line OR
        //value not in the possible of other cells of the same column OR
        //value not in the possible of other cells of the same square
        let res3 = self.resolver3.resolve(g);
        self.nblvl3 += 1;
        if !res3 {
            self.nblvl3ko += 1;
        } else if self.debug {
            let trc = self.resolver3.get_trace();
            println!("{}Level3 => {}", space, trc);
        }
        //x-wing
        let res4 = self.resolver4.resolve(g);
        self.nblvl4 += 1;
        if !res4 {
            self.nblvl4ko += 1;
        } else if self.debug {
            let trc = self.resolver4.get_trace();
            println!("{}Level4 => {}", space, trc);
        }
        g.something_has_some_change()
    }
}
