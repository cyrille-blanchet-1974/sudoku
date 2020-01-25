use super::grid::*;

pub struct Resolver {
    step: u32,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver { step: 0 }
    }

    pub fn go(&mut self, g: &mut Grid, debug: bool) -> bool {
        g.display();
        //if already resolved...
        if !g.is_resolved() {
            //loop until no more to solve
            loop {
                if debug {
                    println!("--------------------step {}--------------------", self.step);
                }
                if !g.resolve() {
                    break;
                }
                if debug {
                    g.display();
                    if !g.is_valid() {
                        println!("Error in the grid!");
                        return false;
                    }
                }
                self.step += 1;
            }
        }
        let res = g.is_resolved();
        if !res {
            //let run level 4 -> tries
            //first made a copy of
            let mut sav = g.clone();
            //second find a unsolved cell
            match g.get_first_unsolved() {
                None => {
                    println!("Strange error: grid not solved by with no unsolved cell (or unsoved cell with no possibles values)");
                    return false;
                }
                Some(val) => {
                    //on the saved grid, let try this value on the cell
                    println!(
                        "Lvl 4-> try value {} on cell l:{}/c:{}",
                        val.2, val.0, val.1
                    );
                    sav.set_val(val.0, val.1, val.2);
                    if self.go(&mut sav, debug) {
                        //we found the solution
                        return true;
                    } else {
                        println!(
                            "Lvl 4-> wrong guess so value {} is not possible for on cell l:{}/c:{}",
                            val.2, val.0, val.1
                        );
                        //wrong guess -> remove this value from the possibles of the original grid and continue
                        g.remove_a_possible(val.0, val.1, val.2);
                        return self.go(g, debug);
                    }
                }
            }
        }
        g.display();
        if res {
            println!("Grid resolved!!!!!");
        } else if debug {
            g.debug();
        }
        res
    }
}
