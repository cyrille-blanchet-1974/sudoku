use super::grid::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use super::cell::CellType;

pub fn read(fic: &str) -> Grid {
    let mut g = Grid::default();
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
                    compute_line(&mut g, line_number, &l);
                } //TODO : else ...
                line_number += 1;
            }
        }
    }
    g
}

fn compute_line(g: &mut Grid, line_number: u8, l: &str) {
    for (col, part) in l.split(',').enumerate() {
        let r: u8 = match part.parse() {
            Err(_) => {
                continue;
            }
            Ok(v) => v,
        };
        let c: u8 = col.try_into().unwrap();
        g.set_val(line_number, c + 1, r, CellType::ORIGIN);
    }
}
