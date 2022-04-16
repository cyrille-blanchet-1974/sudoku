
//the line
pub struct Line {
    known_values: Vec<bool>, //Value already solve in the line
    max: u16,
}

impl Line {
    pub fn new(max: u16) -> Line {
        //add all known values (False at start)
        let mut known = Vec::new();
        for _i in 0..max {
            known.push(false);
        }
        Line {
            known_values: known,
            max,
        }
    }

    /**
     * add a known value to the line
     */
    pub fn add_a_known_value(&mut self, val: u16) {
        if val < 1 {
            return;
        }
        if val > self.max {
            return;
        }
        let val: usize = (val - 1).into();
        self.known_values[val] = true;
    }

    /**
     * is the value already solved in the line
     */
    pub fn is_known(&self, val: u16) -> bool {
        if val < 1 {
            return false;
        }
        if val > self.max {
            return false;
        }
        let val: usize = (val - 1).into();
        self.known_values[val]
    }

    /*
     return remaining values
    */
    pub fn _get_unknown(&self) -> Vec<u16> {
        let mut res = Vec::new();
        for i in 0..self.max {
            let pos: usize = i.into();
            if !self.known_values[pos] {
                res.push(i + 1);
            }
        }
        res
    }
}

#[test]
fn add_a_known_value_test() {
    let mut c = Line::new(9);
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
    for i in 0..=c.max {
        c.add_a_known_value(i);
    }
}

#[test]
fn is_known_test() {
    let mut c = Line::new(9);
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
    let mut c = Line::new(9);
    c.add_a_known_value(1);
    c.add_a_known_value(3);
    assert_eq!(vec!(2, 4, 5, 6, 7, 8, 9), c._get_unknown());
}

impl Clone for Line {
    fn clone(&self) -> Line {
        let mut p = Vec::new();
        for v in &self.known_values {
            p.push(*v);
        }
        Line {
            known_values: p,
            max: self.max,
        }
    }
}

#[test]
fn clone_line_test() {
    let mut ori = Line::new(9);
    ori.add_a_known_value(1);
    ori.add_a_known_value(3);
    ori.add_a_known_value(5);
    ori.add_a_known_value(7);
    ori.add_a_known_value(9);
    let mut copy = ori.clone();
    for i in 1..=9 {
        assert_eq!(ori.is_known(i), copy.is_known(i));
    }
    copy.add_a_known_value(2);
    assert_ne!(ori.is_known(2), copy.is_known(2));
    assert_ne!(ori._get_unknown(), copy._get_unknown());
}
