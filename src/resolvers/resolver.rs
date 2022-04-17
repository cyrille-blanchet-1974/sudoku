use super::super::objects::cell::*;
use super::super::objects::grid::*;
use super::resolver_lvl1::*;
use super::resolver_lvl2::*;
use super::resolver_lvl3::*;
use super::resolver_lvl4::*;
use super::resolver_lvl5::*;

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
    nblvl5: u32,
    nblvl5ko: u32,
    nblvl9guess: u32,
    nblvl9wrongguess: u32,
    resolver1: Option<ResolverLvl1>,
    resolver2: Option<ResolverLvl2>,
    resolver3: Option<ResolverLvl3>,
    resolver4: Option<ResolverLvl4>,
    resolver5: ResolverLvl5,
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
            nblvl5: 0,
            nblvl5ko: 0,
            nblvl9guess: 0,
            nblvl9wrongguess: 0,
            resolver1: None,
            resolver2: None,
            resolver3: None,
            resolver4: None,
            resolver5: ResolverLvl5::new(),
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
            "Called {} times level 5 ({} times with no new result)",
            self.nblvl5, self.nblvl5ko
        );
        println!(
            "Made {} guess at level 9, {} of those were wrong",
            self.nblvl9guess, self.nblvl9wrongguess
        );
    }

    pub fn go(&mut self, g: &mut Grid) -> bool {
        self.resolver1 = Some(ResolverLvl1::new(g.get_metrics().get_square_side()));
        self.resolver2 = Some(ResolverLvl2::new(g.get_metrics().get_square_side()));
        self.resolver3 = Some(ResolverLvl3::new(g.get_metrics().get_square_side()));
        self.resolver4 = Some(ResolverLvl4::new(g.get_metrics().get_square_side()));
        self.gos(g, "".to_string())
    }
    pub fn gos(&mut self, g: &mut Grid, spacer: String) -> bool {
        if self.display {
            g.display();
        }
        //if already resolved...
        if !g.resolved() {
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
                if !g.valid() {
                    if self.debug {
                        println!("Error in the grid!");
                    }
                    return false;
                }
            }
        }
        let res = g.resolved();
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
        let sav_step = self.step - 1;
        self.nblvl9guess += 1;
        //second find a unsolved cell
        match sav.get_a_guess() {
            None => {
                println!("Strange error: grid not solved but with no unsolved cell (or unsoved cell with no possibles values)");
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
                    g.remove_candidate(line, column, guess);
                    self.gos(g, s)
                }
            }
        }
    }

    pub fn resolve(&mut self, g: &mut Grid, space: String) -> bool {
        self.step += 1;
        //try 3 first type of resolution
        //only solution when removing founds of the same line columnand square
        let res1 = self.resolver1.as_ref().unwrap().resolve(g);
        self.nblvl1 += 1;
        if !res1 {
            self.nblvl1ko += 1;
        } else if self.debug {
            let trc = g.get_trace();
            println!("{}Level1 => {}", space, trc);
        }
        //value fount in two other line and two other column of a square
        let res2 = self.resolver2.as_ref().unwrap().resolve(g);
        self.nblvl2 += 1;
        if !res2 {
            self.nblvl2ko += 1;
        } else if self.debug {
            let trc = g.get_trace();
            println!("{}Level2 => {}", space, trc);
        }
        //value not in the possible of other cells of the same line OR
        //value not in the possible of other cells of the same column OR
        //value not in the possible of other cells of the same square
        let res3 = self.resolver3.as_ref().unwrap().resolve(g);
        self.nblvl3 += 1;
        if !res3 {
            self.nblvl3ko += 1;
        } else if self.debug {
            let trc = g.get_trace();
            println!("{}Level3 => {}", space, trc);
        }
        //do not run xwing on grid with side <> 3!!!
        if g.get_metrics().get_square_side() == 3 {
            //x-wing
            let res4 = self.resolver4.as_ref().unwrap().resolve(g);
            self.nblvl4 += 1;
            if !res4 {
                self.nblvl4ko += 1;
            } else if self.debug {
                let trc = g.get_trace();
                println!("{}Level4 => {}", space, trc);
            }
        }
        //remove values in lines or columns when 2 pairs
        let res5 = self.resolver5.resolve(g);
        self.nblvl5 += 1;
        if !res5 {
            self.nblvl5ko += 1;
        } else if self.debug {
            let trc = g.get_trace();
            println!("{}Level5 => {}", space, trc);
        }
        g.something_has_some_change()
    }
}
