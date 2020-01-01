use super::accessor::*;
use super::constant::*;
use std::convert::TryInto;

//State of the cell Resolved or unknown
//we probably will add some hypothesis state when we will have to try some values
enum State {
    Resolved,
    //TODO:   Hypothesis,
    Unknown,
}

//the cell
pub struct Cell {
    state: State,         //its state
    position: usize,      //position in the grid (in the Vec in fact) -> see Map.txt
    column: u8,           //column in the grid 1..9
    line: u8,             //line in the grid 1..9
    square: Cardinal,     //square in the grid
    possibles: Vec<bool>, //possibles values of the cell
    //TODO   hypothesis : u8, // for the future
    answer: u8, //value of the cell when solved
}

impl Cell {
    //construct a cell giving his position in the grid
    pub fn new(pos: usize) -> Cell {
        //add all possibles
        let mut possibles = Vec::new();
        for _i in 0..MAX {
            possibles.push(true);
        }
        //calculate line/column
        let coord = pos_to_coord(pos);
        //then square
        let square = pos_to_square(pos);
        Cell {
            state: State::Unknown,
            position: pos,
            column: coord.1,
            line: coord.0,
            square,
            possibles,
            //TODO           hypothesis : 0,
            answer: 0,
        }
    }

    /**
     * check if resolved
     */
    pub fn is_resolved(&mut self) -> bool {
        match self.state {
            State::Resolved => true,
            _ => false,
        }
    }

    /**
     * get final answer  is resolved
     */
    pub fn get_answer(&self) -> Option<u8> {
        match &self.state {
            State::Resolved => Some(self.answer),
            _ => None,
        }
    }

    /**
     * check if resolved
     * return true if resolved
     */
    fn verify_resolution(&mut self) -> bool {
        if let State::Resolved = self.state {
            return true;
        }
        let mut count = 0; //count of possible left
        let mut val = 1; //val
                         //check all possible
        for i in 1..=MAX {
            let pos = i.try_into().unwrap();
            if self.is_a_possible(pos) {
                count += 1; //one more
                val = i;
            }
        }
        if count == 1 {
            println!(
                "Found a value {}  on cell {} (l:{}/c:{})",
                val, self.position, self.line, self.column
            );
            //if only one possible left
            self.state = State::Resolved; //then cell is resolved
            self.answer = val; //and we got our answer
            return true;
        }
        false
    }

    /**
     * remove a value from the possibles
     * and return true if the cell is resolve
     */
    pub fn remove_a_possible_and_verify(&mut self, val: usize) -> bool {
        match &self.state {
            State::Resolved => true, //println!("cell {} is already solved", self.position),
            State::Unknown => {
                self.remove_a_possible(val);
                return self.verify_resolution();
            }
        }
    }

    /**
     * remove a value from the possibles
     */
    fn remove_a_possible(&mut self, val: usize) {
        self.possibles[val - 1] = false;
    }

    /**
     * is the value a possible
     */
    pub fn is_a_possible(&mut self, val: usize) -> bool {
        self.possibles[val - 1]
    }

    /**
     * return the line of the cell
     */
    pub fn get_line(&self) -> u8 {
        self.line
    }
    /**
     * return the column of the cell
     */
    pub fn get_column(&self) -> u8 {
        self.column
    }
    /**
     * return the square of the cell
     */
    pub fn get_square(&self) -> Cardinal {
        self.square
    }

    fn get_possibles(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        for i in 1..=MAX {
            if self.is_a_possible(i.try_into().unwrap()) {
                res.push(i);
            }
        }
        res
    }

    /*
     set the value of the cell
    */
    pub fn set_val(&mut self, val: u8) {
        if !self.is_a_possible(val.try_into().unwrap()) {
            println!(
                "ERROR! {} is not possible on cell {} (l:{}/c:{})",
                val, self.position, self.line, self.column
            );
            println!("ERROR! remaining possibles: {:?}", self.get_possibles());
            return;
        }
        //remove other possibles
        for i in 1..=MAX {
            if i != val {
                let pos = i.try_into().unwrap();
                self.remove_a_possible(pos);
            }
        }
        //set the answer and change the state
        self.state = State::Resolved;
        self.answer = val;
    }

    /*
      display data of the cell
    */
    pub fn debug(&mut self) {
        let mut poss = Vec::new();
        let mut i = 1;
        for r in &self.possibles {
            if *r {
                poss.push(i);
            }
            i += 1;
        }
        let resolved = self.is_resolved();
        println!(
            "pos:{} resolved:{} possibles:{:?}",
            self.position, resolved, poss
        );
    }
}

#[test]
fn possible_test() {
    let mut c = Cell::new(1);
    for i in 1..MAX + 1 {
        let pos = i.try_into().unwrap();
        assert_eq!(true, c.is_a_possible(pos));
    }
    c.remove_a_possible(5);
    assert_eq!(false, c.is_a_possible(5));
}

#[test]
fn resolution_test() {
    let mut c = Cell::new(1);
    assert_eq!(false, c.is_resolved());
    assert_eq!(None, c.get_answer());
    for v in 1..MAX {
        let val = v.try_into().unwrap();
        c.remove_a_possible_and_verify(val);
    }
    assert_eq!(true, c.is_resolved());
    assert_eq!(Some(9), c.get_answer());
}
