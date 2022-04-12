use super::objects::grid::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(fic: &str, debug: bool) -> Grid {
    let mut g = Grid::default();
    g.set_debug(debug);
    let input = File::open(&fic);
    match input {
        Err(e) => {
            println!("Error reading file {} => {}", &fic, e);
        }
        Ok(f) => {
            let buffered = BufReader::new(f);
            let mut line_number = 1;
            for line in buffered.lines() {
                if let Ok(l) = line {
                    g.compute_line(line_number, &l);
                }
                line_number += 1;
            }
        }
    }
    g
}

pub fn from_vec(data: Vec<String>, debug: bool) -> Grid {
    let mut g = Grid::default();
    g.set_debug(debug);
    let mut line_number = 1;
    for d in data {
        g.compute_line(line_number, &d);
        line_number += 1;
    }
    g
}

pub fn from_vecvec(data: &[Vec<u8>], debug: bool) -> Grid {
    let mut g = Grid::default();
    g.set_debug(debug);
    g.compute_vecvec(data);
    g
}
