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
}

#[test]
fn resolution_test() {
    let g = Grid::new();
    assert_eq!(false, g.is_resolved());
}
