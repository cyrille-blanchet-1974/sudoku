use super::constant::*;
use super::accessor::Cardinal;
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
    position: u8,    //position in the grid (in the Vec in fact) -> see Map.txt
    column: u8,      //column in the grid 1..9
    line: u8,        //line in the grid 1..9
    square: Cardinal,  //square in the grid
    possibles: Vec<bool>,   //possibles values of the cell
    //TODO   hypothesis : u8, // for the future
    answer: u8,         //value of the cell when solved
}

impl Cell {
    //construct a cell giving is position in the grid
    pub fn new(pos: u8) -> Cell {
        //add all possibles
        let mut possibles = Vec::new();
        for _i in 0..MAX {
            possibles.push(true);
        }
        //calculate line/column
        let coord = get_coord(pos);
        //then square
        let square = get_square(coord);
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
     se the value of the cell
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

/*
  from a position calculate line and column
*/
pub fn get_coord(pos: u8) -> (u8, u8) {
    for lin in 1..=LINESIZE {
        for col in 1..=COLUMNSIZE {
            let p = col + (lin - 1) * LINESIZE;
            if p == pos {
                return (lin, col);
            }
        }
    }
    panic!("Position {} not supported", pos);
}
/**
 * check the code that compute line/column from position
 **/
 #[test]
 fn get_coord_test() {
     let c = get_coord(1);
     assert_eq!((1,1), c);
     let c = get_coord(9);
     assert_eq!((1,9), c);
     let c = get_coord(10);
     assert_eq!((2,1), c);
     let c = get_coord(13);
     assert_eq!((2,4), c);
     let c = get_coord(15);
     assert_eq!((2,6), c);
     let c = get_coord(16);
     assert_eq!((2,7), c);
     let c = get_coord(81);
     assert_eq!((9,9), c);
 }

 /*
 from line and column calculate the square
*/
pub fn get_square(coord : (u8, u8))-> Cardinal{
    let res = match coord.0{
        1..=3 => {
            match coord.1{
                1..=3 => 1,
                4..=6 => 2,
                7..=9 => 3,
                _=>0,
            }        
        },
        4..=6 => {
            match coord.1{
                1..=3 => 4,
                4..=6 => 5,
                7..=9 => 6,
                _=>0,
            }        
        },
        7..=9 => {
            match coord.1{
                1..=3 => 7,
                4..=6 => 8,
                7..=9 => 9,
                _=>0,
            }        
        },
        _=>0,
    };
    let tmp = Cardinal::C;
    tmp.from(res)
}
#[test]
fn get_square_test() {
    //Macro (sort of)
    fn local(i:u8)->u8{
        local2(get_coord(i))
    }

    //Macro (sort of)
    fn local2(i:(u8,u8))->u8{
        get_square(i).get_value()
    }    

    assert_eq!(1, local(1));
    assert_eq!(1, local(2));
    assert_eq!(1, local(3));
    assert_eq!(2, local(4));
    assert_eq!(2, local(5));
    assert_eq!(2, local(6));
    assert_eq!(3, local(7));
    assert_eq!(3, local(8));
    assert_eq!(3, local(9));
    assert_eq!(1, local(10));
    assert_eq!(1, local(11));
    assert_eq!(1, local(12));
    assert_eq!(2, local(13));
    assert_eq!(2, local(14));
    assert_eq!(2, local(15));
    assert_eq!(3, local(16));
    assert_eq!(3, local(17));
    assert_eq!(3, local(18));
    assert_eq!(1, local(19));
    assert_eq!(1, local(20));
    assert_eq!(1, local(21));
    assert_eq!(2, local(22));
    assert_eq!(2, local(23));
    assert_eq!(2, local(24));
    assert_eq!(3, local(25));
    assert_eq!(3, local(26));
    assert_eq!(3, local(27));
    
    assert_eq!(1, local2((1,1)));
    assert_eq!(1, local2((2,1)));
    assert_eq!(1, local2((3,1)));
    assert_eq!(1, local2((1,2)));
    assert_eq!(1, local2((2,2)));
    assert_eq!(1, local2((3,2)));
    assert_eq!(1, local2((1,3)));
    assert_eq!(1, local2((2,3)));
    assert_eq!(1, local2((3,3)));
    assert_eq!(5, local2((5,5)));
    assert_eq!(9, local2((9,9)));
}
