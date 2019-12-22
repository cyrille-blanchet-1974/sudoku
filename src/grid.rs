use super::cell::*;
use super::constant::*;
use std::convert::TryInto;

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Grid {
        let mut data = Vec::new();
        for i in 0..GRIDSIZE {
            data.push(Cell::new(i + 1));
        }
        Grid { cells: data }
    }

    pub fn set_val(&mut self, line: u8, column: u8, val: u8) {
        let pos = coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        cell.set_val(val);
    }

    /**
     * check if resolved
     */
    pub fn is_resolved(&self) -> bool {
        for i in 0..GRIDSIZE {
            let pos: usize = i.try_into().unwrap();
            let cell: &Cell = &self.cells[pos];
            if !cell.is_resolved() {
                return false;
            }
        }
        true
    }

    /**
     * check if resolved
     */
    pub fn display(&self) {
        for line in 1..=LINESIZE {
            println!("------------------------------------");
            print!("|");
            for column in 1..=COLUMNSIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                print!(" {} |", cell.get_val());
            }
            println!();
        }
        println!("------------------------------------");
    }
}

fn coord_to_pos(line: u8, column: u8) -> usize {
    let pos = (line - 1) * LINESIZE + column - 1;
    pos.try_into().unwrap()
}

#[test]
fn resolution_test() {
    let g = Grid::new();
    assert_eq!(false, g.is_resolved());
}
