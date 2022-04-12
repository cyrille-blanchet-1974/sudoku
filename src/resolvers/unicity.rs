use super::super::objects::accessor::pos_to_coord;
use super::super::objects::grid::*;
use super::super::objects::cell::CellType;
use super::super::resolvers::resolver::*;
use std::convert::TryInto;
/*
 here we use cpu power to try all possibilities

*/
pub struct VerifyUnicity {
    initial: Grid
}

impl VerifyUnicity {
    pub fn new(initial: Grid) -> VerifyUnicity {
        VerifyUnicity { initial}
    }

    pub fn unique(&mut self) -> bool {
        println!("Solutions for this grid:");
        //first make a copy of the grid and solve it     
        let mut first = self.initial.clone();
        let mut r = Resolver::new(false, false);
        let res = r.go(&mut first);  
        //if not solved, return 0
        if !res { 
            println!("None!!!");
            return false;
        }
        first.display();
        //second loop on cells of type Guess
        for p in self.initial.get_unresolved(){
            let pos: usize = p.try_into().unwrap();
            let answer = first.get_cell(pos).get_answer();
            if answer == None {
                continue;
            }
            let answer = answer.unwrap();
            let coord = pos_to_coord(pos);
            //on a virgin copy try other value for this cell
            for possible in 1..=9{
                if possible == answer {
                    continue;
                }
                let mut second = self.initial.clone();
                second.set_val(coord.0, coord.1, possible, CellType::Origin);
                if second.valid() {
                    //try to resolve the grid
                    let mut r2 = Resolver::new(false, false);
                    let res2 = r2.go(&mut second);
                    //if solve then this grid a more than one solution
                    if res2 {
                        println!("Other Solution:");
                        second.display();
                        println!("May exists other solutions");
                        return false;
                        //we cound continue to display others...?
                    }
                }
            }
            //else continue
        }
        true
    }
}
