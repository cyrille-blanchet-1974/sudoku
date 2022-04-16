use super::cardinal::*;
use super::coordconverter::*;

//type of the cell
#[derive(Debug, Copy, Clone)]
pub enum CellType {
    Unknown,
    Origin,
    Found,
    Guess,
    Xwing,
}
/*
Unknown -> Value not yet  found,
Origin  -> Value given at start
Found   -> Value found by calculation
Guess   -> possible value but not sure yet
Xwing   -> cell member of a X-wing (see lvl4)
*/
impl CellType {
    pub fn get_value(self) -> u8 {
        match self {
            CellType::Unknown => 1,
            CellType::Origin => 2,
            CellType::Found => 3,
            CellType::Guess => 4,
            CellType::Xwing => 5,
        }
    }
}
impl PartialEq for CellType {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
//the cell
pub struct Cell {
    position: usize,      //position in the grid (in the Vec in fact) -> see Map.txt
    column: u16,          //column in the grid 1..max
    line: u16,            //line in the grid 1..max
    square: Cardinal,     //square in the grid
    possibles: Vec<bool>, //possibles values of the cell
    answer: u16,          //value of the cell when solved
    cell_type: CellType,  //type of value
    debug: bool,
    just_resolved: bool,
    possible_removed: bool,
    max: u16,
}

impl Cell {
    /**
     * return trus if we solve the cell or removed at least a possible valur since last call
     */
    pub fn something_has_some_change(&mut self) -> bool {
        let res = self.just_resolved || self.possible_removed;
        self.just_resolved = false;
        self.possible_removed = false;
        res
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    //construct a cell giving his position in the grid
    pub fn new(pos: usize, debug: bool, squareside: u16) -> Cell {
        //add all possibles
        let mut possibles = Vec::new();
        let max = squareside * squareside;
        for _i in 0..max {
            possibles.push(true);
        }
        let c = CoordConverter::new(squareside);
        //calculate line/column
        let coord = c.pos_to_coord(pos);
        //then square
        let square = c.pos_to_square(pos);
        Cell {
            position: pos,
            column: coord.1,
            line: coord.0,
            square,
            possibles,
            //TODO           hypothesis : 0,
            answer: 0,
            cell_type: CellType::Unknown,
            debug,
            just_resolved: false,
            possible_removed: false,
            max,
        }
    }

    /**
     * check if resolved
     */
    pub fn is_resolved(&self) -> bool {
        self.answer != 0
    }

    /**
     * get final answer if is resolved
     */
    pub fn get_answer(&self) -> Option<u16> {
        if self.answer == 0 {
            None
        } else {
            Some(self.answer)
        }
    }

    pub fn get_type(&self) -> CellType {
        self.cell_type
    }
    pub fn set_type(&mut self, t: CellType) {
        self.cell_type = t;
    }

    /**
     * check if resolved
     * return true if resolved
     */
    fn verify_resolution(&mut self) -> bool {
        if self.answer != 0 {
            return true;
        }
        let mut count = 0; //count of possible left
        let mut val = 1; //val
                         //check all possible
        for i in 1..=self.max {
            let pos = i.into();
            if self.candidate(pos) {
                count += 1; //one more
                val = i;
            }
        }
        if count == 1 {
            /*println!(
                "Cell-Found a value {}  on cell {} (l:{}/c:{})",
                val, self.position, self.line, self.column
            );*/
            //if only one possible left
            self.answer = val; //and we got our answer
            self.cell_type = CellType::Found;
            self.just_resolved = true;
            return true;
        }
        false
    }

    /**
     * remove a value from the possibles
     * and return true if the cell is resolve
     */
    pub fn remove_candidate_and_verify(&mut self, val: usize) -> bool {
        if self.answer != 0 {
            true
        } else {
            self.remove_a_possible(val);
            self.verify_resolution()
        }
    }

    /**
     * remove a value from the possibles
     */
    fn remove_a_possible(&mut self, val: usize) {
        if self.possibles[val - 1] {
            self.possible_removed = true;
        }
        self.possibles[val - 1] = false;
    }

    /**
     * is the value a possible
     */
    pub fn candidate(&mut self, val: usize) -> bool {
        self.possibles[val - 1]
    }

    /**
     * return the line of the cell
     */
    pub fn get_line(&self) -> u16 {
        self.line
    }
    /**
     * return the column of the cell
     */
    pub fn get_column(&self) -> u16 {
        self.column
    }
    /**
     * return the square of the cell
     */
    pub fn get_square(&self) -> Cardinal {
        self.square
    }

    /**
     * return a list of possible values for the cell
     */
    pub fn get_possibles(&mut self) -> Vec<u16> {
        let mut res = Vec::new();
        for i in 1..=self.max {
            if self.candidate(i.into()) {
                res.push(i);
            }
        }
        res
    }

    //self.cell_type = CellType::Found;

    /*
     set the value of the cell
    */
    pub fn set_val(&mut self, val: u16, t: CellType) {
        if !self.candidate(val.into()) {
            if self.debug {
                println!(
                    "ERROR! {} is not possible on cell {} (l:{}/c:{})",
                    val, self.position, self.line, self.column
                );
                println!("ERROR! remaining possibles: {:?}", self.get_possibles());
            }
            return;
        }
        //remove other possibles
        for i in 1..=self.max {
            if i != val {
                let pos = i.into();
                self.remove_a_possible(pos);
            }
        }
        //set the answer
        self.answer = val;
        self.cell_type = t;
        if t == CellType::Found {
            self.just_resolved = true;
        }
    }

    /*
      display data of the cell (only if unresolved)
    */
    pub fn debug(&mut self) -> bool {
        if self.is_resolved() {
            print!(
                "  Cell:{} is resolves. Value:{}",
                self.position, self.answer
            );
            return false;
        }
        let mut poss = Vec::new();
        let mut i = 1;
        for r in &self.possibles {
            if *r {
                poss.push(i);
            }
            i += 1;
        }
        print!("  Cell:{} possibles:{:?}", self.position, poss);
        true
    }

    /*
     if the cell only has a pair of possible return it
     else Non
    */
    pub fn get_pair(&self) -> Option<(u16, u16)> {
        if self.is_resolved() {
            return None;
        }
        let mut first = 0;
        let mut second = 0;
        let mut i = 1;
        for r in &self.possibles {
            if *r {
                if first == 0 {
                    first += i;
                } else if second == 0 {
                    second += i;
                } else {
                    //more than 3 possibles...
                    return None;
                }
            }
            i += 1;
        }
        if first == 0 || second == 0 {
            //les than 2 possibles...
            return None;
        }
        //we have a pair
        Some((first, second))
    }
}

#[test]
fn possible_test() {
    let mut c = Cell::new(1, false, 3);
    for i in 1..c.max + 1 {
        let pos = i.into();
        assert_eq!(true, c.candidate(pos));
    }
    c.remove_a_possible(5);
    assert_eq!(false, c.candidate(5));
}

#[test]
fn resolution_test() {
    let mut c = Cell::new(1, false, 3);
    assert_eq!(false, c.is_resolved());
    assert_eq!(None, c.get_answer());
    for v in 1..c.max {
        let val = v.into();
        c.remove_candidate_and_verify(val);
    }
    assert_eq!(true, c.is_resolved());
    assert_eq!(Some(9), c.get_answer());
}

#[test]
fn pair_test() {
    let mut c = Cell::new(1, false, 3);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(1);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(2);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(3);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(4);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(5);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(6);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(7);
    assert_eq!(Some((8, 9)), c.get_pair());
    c.remove_a_possible(8);
    assert_eq!(None, c.get_pair());
    c.remove_a_possible(9);
    assert_eq!(None, c.get_pair());
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        let mut p = Vec::new();
        for v in &self.possibles {
            p.push(*v);
        }
        Cell {
            position: self.position,
            column: self.column,
            line: self.line,
            square: self.square,
            possibles: p,
            answer: self.answer,
            cell_type: self.cell_type,
            debug: self.debug,
            just_resolved: self.just_resolved,
            possible_removed: self.possible_removed,
            max: self.max,
        }
    }
}

#[test]
fn clone_cell_test() {
    let mut ori = Cell::new(1, false, 3);
    ori.remove_a_possible(5);
    ori.debug();
    let mut copy = ori.clone();
    copy.debug();
    assert_eq!(ori.get_possibles(), copy.get_possibles());
    assert_eq!(ori.get_answer(), copy.get_answer());
    assert_eq!(ori.get_column(), copy.get_column());
    assert_eq!(ori.get_line(), copy.get_line());
    assert_eq!(ori.get_square(), copy.get_square());
    ori.set_val(8, CellType::Found);
    assert_ne!(ori.get_possibles(), copy.get_possibles());
}
