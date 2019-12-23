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


#[test]
fn display_test() {
    let mut g = Grid::new();
    g.display();
    g.set_val(1, 1, 1);
    g.set_val(1, 2, 2);
    g.set_val(1, 3, 3);
    g.set_val(1, 4, 4);
    g.set_val(1, 5, 5);
    g.set_val(1, 6, 6);
    g.set_val(1, 7, 7);
    g.set_val(1, 8, 8);
    g.set_val(1, 9, 9);
    g.set_val(2, 1, 4);
    g.set_val(2, 2, 5);
    g.set_val(2, 3, 6);
    g.set_val(2, 4, 7);
    g.set_val(2, 5, 8);
    g.set_val(2, 6, 9);
    g.set_val(2, 7, 1);
    g.set_val(2, 8, 2);
    g.set_val(2, 9, 3);
    g.set_val(3, 1, 7);
    g.set_val(3, 2, 8);
    g.set_val(3, 3, 9);
    g.set_val(3, 4, 1);
    g.set_val(3, 5, 2);
    g.set_val(3, 6, 3);
    g.set_val(3, 7, 4);
    g.set_val(3, 8, 5);
    g.set_val(3, 9, 6);
    g.set_val(4, 1, 2);
    g.set_val(4, 2, 3);
    g.set_val(4, 3, 4);
    g.set_val(4, 4, 5);
    g.set_val(4, 5, 6);
    g.set_val(4, 6, 7);
    g.set_val(4, 7, 8);
    g.set_val(4, 8, 9);
    g.set_val(4, 9, 1);
    g.set_val(5, 1, 5);
    g.set_val(5, 2, 6);
    g.set_val(5, 3, 7);
    g.set_val(5, 5, 8);
    g.set_val(5, 6, 9);
    g.set_val(5, 7, 1);
    g.set_val(5, 8, 2);
    g.set_val(5, 9, 3);
    g.set_val(6, 1, 4);
    g.set_val(6, 2, 8);
    g.set_val(6, 3, 9);
    g.set_val(6, 4, 1);
    g.set_val(6, 5, 2);
    g.set_val(6, 6, 3);
    g.set_val(6, 7, 4);
    g.set_val(6, 8, 5);
    g.set_val(6, 9, 6);
    g.set_val(7, 1, 7);
    g.set_val(7, 2, 3);
    g.set_val(7, 3, 4);
    g.set_val(7, 4, 5);
    g.set_val(7, 5, 6);
    g.set_val(7, 6, 7);
    g.set_val(7, 7, 8);
    g.set_val(7, 8, 9);
    g.set_val(7, 9, 1);
    g.set_val(8, 1, 2);
    g.set_val(8, 2, 6);
    g.set_val(8, 3, 7);
    g.set_val(8, 4, 8);
    g.set_val(8, 5, 9);
    g.set_val(8, 6, 1);
    g.set_val(8, 7, 2);
    g.set_val(8, 8, 3);
    g.set_val(8, 9, 4);
    g.set_val(9, 1, 5);
    g.set_val(9, 2, 6);
    g.set_val(9, 3, 9);
    g.set_val(9, 4, 1);
    g.set_val(9, 5, 2);
    g.set_val(9, 6, 3);
    g.set_val(9, 7, 4);
    g.set_val(9, 8, 5);
    g.set_val(9, 9, 6);
    g.display();
}
