use std::collections::HashMap;
use super::constant::*;
use std::convert::TryInto;

pub enum Cardinal{
    N,
    S,
    E,
    W,
    C,
    NE,
    NW,
    SE,
    SW
} 
impl Cardinal {
    pub fn get_value(&self)->u8{
        match self {
            Cardinal::NW => 0,
            Cardinal::N => 1,
            Cardinal::NE => 2,
            Cardinal::W => 3,
            Cardinal::C=> 4,
            Cardinal::E=>5,
            Cardinal::SW=>6,
            Cardinal::S=>7,
            Cardinal::SE=>8,
        }
    }
}

pub struct Accessor {
    lines: HashMap<u8, Vec<u8>>, 
    columns: HashMap<u8, Vec<u8>>,
    squares: HashMap<u8,Vec<u8>>,
}

impl Accessor {
    pub fn new() -> Accessor {
        Accessor{
            lines : gen_lines(),
            columns : HashMap::new(),
            squares : HashMap::new(),
        }
    }
}

fn gen_lines()-> HashMap<u8, Vec<u8>>{
    let mut res = HashMap::new(); 
    for i in 0..LINESIZE {
        res.insert(i,gen_line(i));
    };
    res
}

fn gen_line(l : u8)->Vec<u8>{
    let mut res = Vec::new();
    let mut pos = l*LINESIZE;
    for i in 1..LINESIZE {
        res.push(pos);
        pos+=1; 
    }
    res
}



#[test]
fn lines_test() {
    let l = gen_lines();
    //TODO assert_eq!(l.get(0), Some(vec!(0,1,2,3,4,5,6,7,8)));
    //TODO assert_eq!(l.get(8), Some(vec!(73,74,75,76,77,78,79,80,81)));
}
