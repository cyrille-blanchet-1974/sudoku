use super::cardinal::*;
use super::coordconverter::*;
use std::collections::HashMap;

pub struct Accessor {
    lines: HashMap<u16, Vec<u16>>,
    columns: HashMap<u16, Vec<u16>>,
    squares: HashMap<u16, Vec<u16>>,
    pub coordconverter: CoordConverter,
}

impl Accessor {
    pub fn new(squareside: u16) -> Accessor {
        Accessor {
            lines: gen_lines(squareside * squareside),
            columns: gen_columns(squareside * squareside),
            squares: gen_squares(squareside),
            coordconverter: CoordConverter::new(squareside),
        }
    }

    pub fn get_line(&self, l: u16) -> Vec<u16> {
        match self.lines.get(&l) {
            None => Vec::new(),
            Some(x) => x.clone(),
        }
    }
    pub fn get_column(&self, c: u16) -> Vec<u16> {
        match self.columns.get(&c) {
            None => Vec::new(),
            Some(x) => x.clone(),
        }
    }
    pub fn get_square(&self, s: Cardinal) -> Vec<u16> {
        match self.squares.get(&(s.get_value())) {
            None => Vec::new(),
            Some(x) => x.clone(),
        }
    }
}

fn gen_squares(squareside: u16) -> HashMap<u16, Vec<u16>> {
    let mut res = HashMap::new();
    let mut i = 1;
    for l in 0..squareside {
        for c in 0..squareside {
            res.insert(
                i,
                gen_square(
                    l * squareside,
                    (l + 1) * squareside,
                    c * squareside,
                    (c + 1) * squareside,
                    squareside * squareside,
                ),
            );
            i += 1;
        }
    }
    res
}

fn gen_square(l1: u16, l2: u16, c1: u16, c2: u16, max: u16) -> Vec<u16> {
    let mut res = Vec::new();
    for line in l1..l2 {
        for column in c1..c2 {
            res.push(line * max + column);
        }
    }
    res
}

fn gen_lines(max: u16) -> HashMap<u16, Vec<u16>> {
    let mut res = HashMap::new();
    for i in 0..max {
        res.insert(i + 1, gen_line(i, max));
    }
    res
}

fn gen_line(l: u16, max: u16) -> Vec<u16> {
    let mut res = Vec::new();
    let mut pos = l * max;
    for _i in 0..max {
        res.push(pos);
        pos += 1;
    }
    res
}

fn gen_columns(max: u16) -> HashMap<u16, Vec<u16>> {
    let mut res = HashMap::new();
    for i in 0..max {
        res.insert(i + 1, gen_column(i, max));
    }
    res
}

fn gen_column(c: u16, max: u16) -> Vec<u16> {
    let mut res = Vec::new();
    let mut pos = c;
    for _i in 0..max {
        res.push(pos);
        pos += max;
    }
    res
}

#[test]
fn lines_test() {
    assert_eq!(gen_line(0, 9), vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));
    assert_eq!(gen_line(8, 9), vec!(72, 73, 74, 75, 76, 77, 78, 79, 80));
    let l = gen_lines(9);
    assert_eq!(l.get(&1).unwrap(), &vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));
    assert_eq!(
        l.get(&9).unwrap(),
        &vec!(72, 73, 74, 75, 76, 77, 78, 79, 80)
    );
}

#[test]
fn columns_test() {
    assert_eq!(gen_column(0, 9), vec!(0, 9, 18, 27, 36, 45, 54, 63, 72));
    assert_eq!(gen_column(8, 9), vec!(8, 17, 26, 35, 44, 53, 62, 71, 80));
    let l = gen_columns(9);
    assert_eq!(l.get(&1).unwrap(), &vec!(0, 9, 18, 27, 36, 45, 54, 63, 72));
    assert_eq!(l.get(&9).unwrap(), &vec!(8, 17, 26, 35, 44, 53, 62, 71, 80));
}

#[test]
fn squares_test() {
    assert_eq!(
        gen_square(0, 3, 0, 3, 9),
        vec!(0, 1, 2, 9, 10, 11, 18, 19, 20)
    );
    assert_eq!(
        gen_square(3, 6, 3, 6, 9),
        vec!(30, 31, 32, 39, 40, 41, 48, 49, 50)
    );
    assert_eq!(
        gen_square(3, 6, 0, 3, 9),
        vec!(27, 28, 29, 36, 37, 38, 45, 46, 47)
    );
    let l = gen_squares(3);
    assert_eq!(l.get(&1).unwrap(), &vec!(0, 1, 2, 9, 10, 11, 18, 19, 20));
    assert_eq!(
        l.get(&4).unwrap(),
        &vec!(27, 28, 29, 36, 37, 38, 45, 46, 47)
    );
}
