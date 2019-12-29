use super::constant::*;
use super::accessor::*;
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
    state: State,    //its state 
    position: usize,    //position in the grid (in the Vec in fact) -> see Map.txt
    column: u8,      //column in the grid 1..9
    line: u8,        //line in the grid 1..9
    square: Cardinal,  //square in the grid
    possibles: Vec<bool>,   //possibles values of the cell
    //TODO   hypothesis : u8, // for the future
    answer: u8,         //value of the cell when solved
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
    pub fn is_resolved(&self) -> bool {
        match &self.state {
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
     */
    fn verify_resolvution(&mut self) {
        let mut count = 0;//count of possible left
        let mut val = 1; //val
        //check all possible
        for i in 1..=MAX {
            let pos = i.try_into().unwrap();
            if self.is_a_possible(pos) {
                count += 1; //one more
                val = i; 
            }
        }
        if count == 1 { //if only one possible left
            self.state = State::Resolved; //then cell is resolved
            self.answer = val; //and we got our answer
        }
    }

    /**
     * remove a value from the possibles
     */
    pub fn remove_a_possible(&mut self, val: usize) {
        match &self.state {
            State::Resolved => {},//println!("cell {} is already solved", self.position),
            State::Unknown => {
                self.possibles[val - 1] = false;
                self.verify_resolvution();
            },
        };
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
    pub fn get_square(&self)-> Cardinal {
        self.square
    }
    /*
     set the value of the cell
    */
    pub fn set_val(&mut self, val: u8) {
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
    pub fn debug(&self) {
        let mut poss = Vec::new();
        let mut i =1;
        for r in &self.possibles{
            if *r {poss.push(i);}
            i+=1;
        }
        println!("pos:{} resolved:{} possibles:{:?}",self.position,self.is_resolved(),poss);
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
    for i in 1..MAX {
        let pos = i.try_into().unwrap();
        c.remove_a_possible(pos);
    }
    assert_eq!(true, c.is_resolved());
    assert_eq!(Some(9), c.get_answer());
}