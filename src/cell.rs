use super::constant::*;
use super::accessor::Cardinal;
use std::convert::TryInto;

enum State {
    Resolved,
    //TODO:   Hypothesis,
    Unknown,
}

pub struct Cell {
    state: State,
    position: u8, //position in the grid
    column: u8,
    line: u8,
    square: Cardinal,
    possibles: Vec<bool>,
    //TODO   hypothesis : u8,
    answer: u8,
}

impl Cell {
    pub fn new(pos: u8) -> Cell {
        let mut possibles = Vec::new();
        for _i in 0..MAX {
            possibles.push(true);
        }
        let coord = get_coord(pos);
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
     * get answer
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
    fn verify_resolve(&mut self) {
        let mut count = 0;
        let mut val = 1;
        for i in 1..=MAX {
            let pos = i.try_into().unwrap();
            if self.is_a_possible(pos) {
                count += 1;
                val = i;
            }
        }
        if count == 1 {
            self.state = State::Resolved;
            self.answer = val;
        }
    }

    /**
     * remove a value from the possibles
     */
    pub fn remove_a_possible(&mut self, val: usize) {
        match &self.state {
            State::Resolved => return,//println!("cell {} is already solved", self.position),
            State::Unknown => {
                self.possibles[val - 1] = false;
                self.verify_resolve();
            },
        };
    }

    /**
     * is the value val a possible
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
    pub fn get_column(&self) -> u8 {
        self.column
    }
    pub fn get_square(&self)-> Cardinal {
        self.square
    }
    pub fn get_val(&self) -> u8 {
        match &self.state {
            State::Resolved => self.answer,
            _ => 0,
        }
    }
    pub fn set_val(&mut self, val: u8) {
        for i in 1..=MAX {
            if i != val {
                let pos = i.try_into().unwrap();
                self.remove_a_possible(pos);
            }
        }
        self.state = State::Resolved;
        self.answer = val;
    }
    pub fn debug(&self) {
        println!("pos:{} resolved:{} possibles:{:?}",self.position,self.is_resolved(),self.possibles);
    }
}

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
fn get_line_test() {
    let c = Cell::new(1);
    assert_eq!(1, c.get_line());
    let c = Cell::new(9);
    assert_eq!(1, c.get_line());
    let c = Cell::new(10);
    assert_eq!(2, c.get_line());
    let c = Cell::new(15);
    assert_eq!(2, c.get_line());
    let c = Cell::new(81);
    assert_eq!(9, c.get_line());
    let c = Cell::new(13);
    assert_eq!(2, c.get_line());
}

#[test]
fn get_column_test() {
    let c = Cell::new(1);
    assert_eq!(1, c.get_column());
    let c = Cell::new(9);
    assert_eq!(9, c.get_column());
    let c = Cell::new(10);
    assert_eq!(1, c.get_column());
    let c = Cell::new(16);
    assert_eq!(7, c.get_column());
    let c = Cell::new(81);
    assert_eq!(9, c.get_column());
    let c = Cell::new(13);
    assert_eq!(4, c.get_column());
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

#[test]
fn get_square_test() {
    assert_eq!(1, get_square(get_coord(1)).get_value());
    assert_eq!(1, get_square(get_coord(2)).get_value());
    assert_eq!(1, get_square(get_coord(3)).get_value());
    assert_eq!(2, get_square(get_coord(4)).get_value());
    assert_eq!(2, get_square(get_coord(5)).get_value());
    assert_eq!(2, get_square(get_coord(6)).get_value());
    assert_eq!(3, get_square(get_coord(7)).get_value());
    assert_eq!(3, get_square(get_coord(8)).get_value());
    assert_eq!(3, get_square(get_coord(9)).get_value());
    assert_eq!(1, get_square(get_coord(10)).get_value());
    assert_eq!(1, get_square(get_coord(11)).get_value());
    assert_eq!(1, get_square(get_coord(12)).get_value());
    assert_eq!(2, get_square(get_coord(13)).get_value());
    assert_eq!(2, get_square(get_coord(14)).get_value());
    assert_eq!(2, get_square(get_coord(15)).get_value());
    assert_eq!(3, get_square(get_coord(16)).get_value());
    assert_eq!(3, get_square(get_coord(17)).get_value());
    assert_eq!(3, get_square(get_coord(18)).get_value());
    assert_eq!(1, get_square(get_coord(19)).get_value());
    assert_eq!(1, get_square(get_coord(20)).get_value());
    assert_eq!(1, get_square(get_coord(21)).get_value());
    assert_eq!(2, get_square(get_coord(22)).get_value());
    assert_eq!(2, get_square(get_coord(23)).get_value());
    assert_eq!(2, get_square(get_coord(24)).get_value());
    assert_eq!(3, get_square(get_coord(25)).get_value());
    assert_eq!(3, get_square(get_coord(26)).get_value());
    assert_eq!(3, get_square(get_coord(27)).get_value());

    
    assert_eq!(1, get_square((1,1)).get_value());
    assert_eq!(1, get_square((2,1)).get_value());
    assert_eq!(1, get_square((3,1)).get_value());
    assert_eq!(1, get_square((1,2)).get_value());
    assert_eq!(1, get_square((2,2)).get_value());
    assert_eq!(1, get_square((3,2)).get_value());
    assert_eq!(1, get_square((1,3)).get_value());
    assert_eq!(1, get_square((2,3)).get_value());
    assert_eq!(1, get_square((3,3)).get_value());
    assert_eq!(5, get_square((5,5)).get_value());
    assert_eq!(9, get_square((9,9)).get_value());
}
