use super::accessor::*;
use super::cell::*;
use super::constant::*;
use super::grid::*;
use std::convert::TryInto;

pub struct ResolverLvl4 {
    acc: Accessor, //methods to retreive cells by coordinates
}

impl ResolverLvl4 {
    pub fn new() -> ResolverLvl4 {
        ResolverLvl4 {
            acc: Accessor::new(),
        }
    }

    /*
    X-wing resolve
    return true if found one or more
    */
    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        print!("Lvl4->");
        let mut solve_one_at_least = false;
        if self.resolve_line(g){
            solve_one_at_least=true;
        }
        if self.resolve_column(g){
            solve_one_at_least=true;
        }
        println!();
        solve_one_at_least
    }

    fn resolve_line(&mut self, g: &mut Grid) -> bool {
        //loop values
        for v in 1..=MAX {
            let val: usize = v.try_into().unwrap();
            //loop lines
            for line in 1..=COLUMNSIZE {
                for p in self.acc.get_line(line) {
                    let pos: usize = p.try_into().unwrap();
                    let cell: &mut Cell = &mut (g.get_cell(pos));
                    if cell.is_a_possible(val) {
                    }
                }
                        //check columns with value resolved if only 2 columns keep
                //if 2 lines (and only two) with same two cols remove val off possible for other cols of the two lines
            }
        }
        false
    }
    fn resolve_column(&mut self, g: &mut Grid) -> bool {
        //loop values
        for v in 1..=MAX {
            let val: usize = v.try_into().unwrap();
            //loop columns
            for column in 1..=LINESIZE {
                for p in self.acc.get_column(column) {
                    let pos: usize = p.try_into().unwrap();
                    let cell: &mut Cell = &mut (g.get_cell(pos));
                    if cell.is_a_possible(val) {
                    }
                }

                //check lines with value resolved if only 2 lines keep
                //if 2 columns (and only two) with same two lines remove val off possible for other lines of the two cols
            }
        }
        false
    }

}
