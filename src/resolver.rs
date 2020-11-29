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
}

impl Resolver {
    pub fn new() -> Resolver {
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

    pub fn go(&mut self, g: &mut Grid, debug: bool, display: bool) -> bool {
        if display {
            g.display();
        }
        //if already resolved...
        if !g.is_resolved() {
            //loop until no more to solve
            loop {
                if debug {
                    println!("--------------------step {}--------------------", self.step);
                }
                if !self.resolve(g,debug) {
                    break;
                }
                if display {
                    g.display();
                }
                if !g.is_valid() {
                    if debug {
                        println!("Error in the grid!");
                    }
                    return false;
                }
            }
        }
        let res = g.is_resolved();
        if !res {
            return self.resolve_lvl9(g, debug, display);
        }
        if display {
            g.display();            
        }
        if !res {
            g.debug();
        }
        res
    }

    fn resolve_lvl9(&mut self, g: &mut Grid, debug: bool, display: bool) -> bool {
        //let run level 9 -> guesses
        //first made a copy of
        let mut sav = g.clone();
        let sav_step = self.step;
        self.nblvl9guess += 1;
        //second find a unsolved cell
        match sav.get_first_unsolved() {
            None => {
                println!("Strange error: grid not solved by with no unsolved cell (or unsoved cell with no possibles values)");
                false
            }
            Some(val) => {
                //on the saved grid, let try this value on the cell
                if debug {
                    println!("Lvl9-> try value {} on cell l:{}/c:{}", val.2, val.0, val.1);
                }
                /*if display && debug {
                    g.debug();
                }
                if display && debug {
                    sav.debug();
                }*/
                sav.set_val(val.0, val.1, val.2, CellType::GUESS);
                if self.go(&mut sav, debug, display) {
                    //we found the solution
                    g.copy_from(sav);
                    true
                } else {
                    if debug {
                        println!(
                            "Lvl9-> wrong guess so value {} is not possible for on cell l:{}/c:{} -> restoring previous grid (from step {})",
                            val.2, val.0, val.1,sav_step
                        );
                    }
                    self.nblvl9wrongguess += 1;
                    //wrong guess -> remove this value from the possibles of the original grid and continue
                    g.remove_a_possible(val.0, val.1, val.2);
                    self.go(g, debug, display)
                }
            }
        }
    }
/*
    pub fn resolve(&mut self, g: &mut Grid, debug: bool) -> bool {
        loop{
            let mut l1 = ResolverLvl1::new(debug);
            let res1 = l1.resolve(g);
            print!("L1");
            self.nblvl1 += 1;
            if !res1 {
                self.nblvl1ko += 1;
            }
            if !g.something_has_some_change() {
                loop{
                    let mut l2 = ResolverLvl2::new(debug);
                    let res2 = l2.resolve(g);
                    print!("L2");
                    self.nblvl2 += 1;
                    if !res2 {
                        self.nblvl2ko += 1;
                    }
                    if !g.something_has_some_change() {
                        loop{  
                            let mut l3 = ResolverLvl3::new(debug);
                            let res3 = l3.resolve(g);
                            print!("L3");
                            self.nblvl3 += 1;
                            if !res3 {
                                self.nblvl3ko += 1;
                            }
                            if !g.something_has_some_change() {
                                let mut l4 = ResolverLvl4::new(debug);
                                let res4 = l4.resolve(g);
                                print!("L4");
                                self.nblvl4 += 1;
                                if !res4 {
                                    self.nblvl4ko += 1;
                                }
                                if !g.something_has_some_change() {
                                    break;
                                }                        
                            }//l3 found nothing                           
                        }//loop L3
                    }//l2 found nothing
                    if !g.something_has_some_change() {
                        break;
                    }                    
                }//loop L2
            }//L1 found nothing
            if !g.something_has_some_change() {
                break;
            }                    
        }//lopp L1
        false
    }*/
    pub fn resolve(&mut self, g: &mut Grid, debug: bool) -> bool {
        self.step += 1;
        //try 3 first type of resolution
        //only solution when removing founds of the same line columnand square
        let mut l1 = ResolverLvl1::new(debug);
        let res1 = l1.resolve(g);
        self.nblvl1 += 1;
        if !res1 {
            self.nblvl1ko += 1;
        }
        /*if g.something_has_some_change() || res1{
            println!("lvl1 found something");
            if res1 {
                println!("lvl1 returned something");
            }
        }*/
        //value fount in two other line and two other column of a square
        let mut l2 = ResolverLvl2::new(debug);
        let res2 = l2.resolve(g);
        self.nblvl2 += 1;
        if !res2 {
            self.nblvl2ko += 1;
        }
        /*if g.something_has_some_change() || res2{
            println!("lvl2 found something");
            if res2 {
                println!("lvl2 returned something");
            }
        }*/
        //value not in the possible of other cells of the same line OR
        //value not in the possible of other cells of the same column OR
        //value not in the possible of other cells of the same square
        let mut l3 = ResolverLvl3::new(debug);
        let res3 = l3.resolve(g);
        self.nblvl3 += 1;
        if !res3 {
            self.nblvl3ko += 1;
        }
        /*if g.something_has_some_change() || res3{
            println!("lvl3 found something");
            if res3 {
                println!("lvl3 returned something");
            }
        }*/
        //x-wing
        let mut l4 = ResolverLvl4::new(debug);
        let res4 = l4.resolve(g);
        self.nblvl4 += 1;
        if !res4 {
            self.nblvl4ko += 1;
        }
        /*if g.something_has_some_change() || res4{
            println!("lvl4 found something");
            if res4 {
                println!("lvl4 returned something");
            }
        }
        res1 || res2 || res3 || res4*/
        g.something_has_some_change()
    }
}
