use super::constant::*;
use std::convert::TryInto;

//the column
pub struct Column {
    position: u8,    //position in the grid 0..LINESIZE
    known_values: Vec<bool>,   //Value already solve in the line
}

impl Column {
    //construct a column giving his position
    pub fn new(pos: u8) -> Column {
        //add all known values (False at start)
        let mut known = Vec::new();
        for _i in 0..MAX {
            known.push(false);
        }
        Column {
            position: pos,
            known_values: known,
        }
    }

    /**
     * add a known value to the column
     */
    pub fn add_a_known_value(&mut self, val: u8) {
        if val < 1 { return;}
        if val > MAX { return;}
        let val:usize = (val-1).try_into().unwrap();
        self.known_values[val] = true;
    }

    /**
     * is the value already solved in the column
     */
    pub fn is_known(&self, val: u8) -> bool {
        if val < 1 { return false;}
        if val > MAX { return false;}
        let val:usize = (val-1).try_into().unwrap();
        self.known_values[val]
    }

    /*
     return remaining values
    */
    pub fn get_unknown(&self)->Vec<u8> {
        let mut res = Vec::new();
        for i in 0..MAX {
            let pos:usize = i.try_into().unwrap();
            if !self.known_values[pos] {
                res.push(i+1);
            }
        }
        res
    }
}

#[test]
fn add_a_known_value_test() {
    let mut c = Column::new(1);
    c.add_a_known_value(1);
    c.add_a_known_value(2);
    c.add_a_known_value(3);
    c.add_a_known_value(4);
    c.add_a_known_value(5);
    c.add_a_known_value(6);
    c.add_a_known_value(7);
    c.add_a_known_value(8);
    c.add_a_known_value(9);
    c.add_a_known_value(10);
    c.add_a_known_value(0);
    for i in 0..=MAX {
        c.add_a_known_value(i);
    }
}


#[test]
fn is_known_test() {
    let mut c = Column::new(1);
    c.add_a_known_value(1);
    c.add_a_known_value(3);
    assert_eq!(true, c.is_known(1));
    assert_eq!(true, c.is_known(3));
    assert_eq!(false, c.is_known(0));
    assert_eq!(false, c.is_known(2));
    assert_eq!(false, c.is_known(4));
    assert_eq!(false, c.is_known(5));
    assert_eq!(false, c.is_known(6));
    assert_eq!(false, c.is_known(7));
    assert_eq!(false, c.is_known(8));
    assert_eq!(false, c.is_known(9));
    assert_eq!(false, c.is_known(11));
}

#[test]
fn get_unknown_test() {
    let mut c = Column::new(1);
    c.add_a_known_value(1);
    c.add_a_known_value(3);
    assert_eq!(vec!(2,4,5,6,7,8,9), c.get_unknown());
}