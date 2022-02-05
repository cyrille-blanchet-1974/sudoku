use super::accessor::pos_to_coord;
use super::grid::*;
use super::resolver::*;
use std::convert::TryInto;
/*
 here we use cpu power to try all possibilities

*/
pub struct VerifyUnicity {
    debug: bool,
    initial: Grid
}

impl VerifyUnicity {
    pub fn new(debug: bool, g: Grid) -> VerifyUnicity {
        VerifyUnicity { debug, initial: g }
    }

    //display the grid
    /*pub fn display(&mut self, g: &mut Grid) {
        g.display_bw();
    }*/

    pub fn is_unique(&mut self) -> u8 {
        if self.debug {
            println!("Solutions for this grid:");
        }
        let mut nb=0;
        //first make a copy of the grid and solve it     
        let mut first = self.initial.clone();
        let mut r = Resolver::new(self.debug, false);
        let res = r.go(&mut first);  
        //if not solved, return 0
        if !res { 
            println!("None!!!");
            return nb;
        }
        if self.debug {
            first.display();
        }
        nb +=1;
        //second loop on cells of type Guess
        for p in self.initial.get_unresolved(){
            let pos: usize = p.try_into().unwrap();
            let answer = first.get_cell(pos).get_answer();
            //on a virgin copy remove the answer of this cell of the possibles values
            let mut second = self.initial.clone();
            let coord = pos_to_coord(pos);
            second.remove_a_possible(coord.0,coord.1,answer.unwrap());
            //try to resolve the grid
            let mut r2 = Resolver::new(self.debug, false);
            let res2 = r2.go(&mut second);
            //if solve then this grid a more than one solution
            if res2 {
                if self.debug {
                    second.display();
                }
                nb +=1;
                //we cntinue to display others...
            }
            //else continue
        }
        nb
    }

}
