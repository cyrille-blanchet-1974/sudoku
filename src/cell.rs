use super::constant::*;

pub enum State {
    Resolve(u8),
    Hypothesis(u8),
    Unknown(Vec<u8>),
}

pub struct Cell {
    state: State,
    position: u8, //position in the grid
    column : u8,
    line : u8,
}

impl Cell {
    pub fn new(pos :u8) -> Cell {
        let mut possibles = Vec::new();
        for i in 1..9{
            possibles.push(i);            
        }
        let coord = get_coord(pos);
        Cell {
            state: State::Unknown(possibles),
            position: pos,
            column : coord.1,
            line: coord.0,
        }
    }

    /**
     * return the line of the cell
     */
    pub fn get_line(&self)->u8
    {
        self.line
    }
    pub fn get_column(&self)->u8
    {
        self.column
    }
}

pub fn get_coord(pos : u8)->(u8,u8)
{
    for lin in 1..LINESIZE+1
    {
        for col in 1..COLUMNSIZE+1
        {
            let p = col + (lin-1) * LINESIZE; 
            if p == pos {
                return (lin,col);
            }
        }
    }
    panic!("Position {} not supported",pos);
}


#[test]
fn get_line_test() {
    let c = Cell::new(1);
    assert_eq!(1,c.get_line());
    let c = Cell::new(9);
    assert_eq!(1,c.get_line());
    let c = Cell::new(10);
    assert_eq!(2,c.get_line());
    let c = Cell::new(15);
    assert_eq!(2,c.get_line());
    let c = Cell::new(81);
    assert_eq!(9,c.get_line());
}


#[test]
fn get_column_test() {
    let c = Cell::new(1);
    assert_eq!(1,c.get_column());
    let c = Cell::new(9);
    assert_eq!(9,c.get_column());
    let c = Cell::new(10);
    assert_eq!(1,c.get_column());
    let c = Cell::new(16);
    assert_eq!(7,c.get_column());
    let c = Cell::new(81);
    assert_eq!(9,c.get_column());
}
