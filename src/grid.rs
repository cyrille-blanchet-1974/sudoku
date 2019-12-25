use super::accessor::*;
use super::cell::*;
use super::constant::*;
use std::convert::TryInto;

pub struct Grid {
    cells: Vec<Cell>,
    acc: Accessor,
}

impl Grid {
    pub fn new() -> Grid {
        let mut data = Vec::new();
        for i in 0..GRIDSIZE {
            data.push(Cell::new(i + 1));
        }
        Grid { 
            cells: data,
            acc: Accessor::new(),
         }
    }

    pub fn set_val(&mut self, line: u8, column: u8, val: u8) {
        let pos = coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        cell.set_val(val);
        println!("setting value {} on cell[{}]",val,pos);
        //TODO: remove this value from possibles of the line, the column and the square
        let lin = self.acc.get_line(cell.get_line());
        let col = self.acc.get_column(cell.get_column());
        let squ = self.acc.get_square(cell.get_square());
        /*
        for c in lin{
            let cc:usize = c.try_into().unwrap();
            if cc != pos {
                let cell: &mut Cell = &mut (self.cells[cc]);
                if cell.is_a_possible(val.try_into().unwrap()) {
                    println!("(line] removing value value {} from the possible of on cell[{}]",val,cc);
                    cell.remove_a_possible(val.try_into().unwrap());
                }
            }
        }
        for c in col{
            let cc:usize = c.try_into().unwrap();
            if cc != pos {
                let cell: &mut Cell = &mut (self.cells[cc]);
                if cell.is_a_possible(val.try_into().unwrap()) {
                    println!("[column] removing value value {} from the possible of on cell[{}]",val,cc);
                    cell.remove_a_possible(val.try_into().unwrap());
                }
            }
        }*/
        for c in squ{
            let cc:usize = c.try_into().unwrap();
            if cc != pos {
               let cell: &mut Cell = &mut (self.cells[cc]);
                if cell.is_a_possible(val.try_into().unwrap()) {
                    println!("[square] removing value value {} from the possible of on cell[{}]",val,cc);
                    cell.remove_a_possible(val.try_into().unwrap());
                }
            }
        }
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
        if self.is_resolved() {
            println!("Puzzle soled!");
        }
    }

    pub fn check_puzzle(&self) -> bool {
        let attendu = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1;
        let mut c = 0;
        //ctl by line
        for line in 1..=LINESIZE {
            c = 0;
            for column in 1..=COLUMNSIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                c += cell.get_val();
            }
            if c != attendu {
                println!("unckeck line {} => {}", line, c);
                return false;
            }
        }
        //ctl by column
        for column in 1..=COLUMNSIZE {
            c = 0;
            for line in 1..=LINESIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                c += cell.get_val();
            }
            if c != attendu {
                println!("unckeck column {} => {}", column, c);
                return false;
            }
        }
        //ctl by square
        if !self.check_square(1, 3, 1, 3) {
            return false;
        }
        if !self.check_square(4, 6, 1, 3) {
            return false;
        }
        if !self.check_square(7, 9, 1, 3) {
            return false;
        }
        if !self.check_square(1, 3, 4, 6) {
            return false;
        }
        if !self.check_square(4, 6, 4, 6) {
            return false;
        }
        if !self.check_square(7, 9, 4, 6) {
            return false;
        }
        if !self.check_square(1, 3, 7, 9) {
            return false;
        }
        if !self.check_square(4, 6, 7, 9) {
            return false;
        }
        if !self.check_square(7, 9, 7, 9) {
            return false;
        }
        return true;
    }
    fn check_square(&self, l1: u8, l2: u8, c1: u8, c2: u8) -> bool {
        let attendu = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1;
        let mut c = 0;
        for column in c1..=c2 {
            for line in l1..=l2 {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                c += cell.get_val();
            }
        }
        if c != attendu {
            println!("uncheck square {},{} {},{} => {}", c1, c2, l1, l2, c);
            return false;
        }
        return true;
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
    g.set_val(5, 4, 8);
    g.set_val(5, 5, 9);
    g.set_val(5, 6, 1);
    g.set_val(5, 7, 2);
    g.set_val(5, 8, 3);
    g.set_val(5, 9, 4);
    g.set_val(6, 1, 8);
    g.set_val(6, 2, 9);
    g.set_val(6, 3, 1);
    g.set_val(6, 4, 2);
    g.set_val(6, 5, 3);
    g.set_val(6, 6, 4);
    g.set_val(6, 7, 5);
    g.set_val(6, 8, 6);
    g.set_val(6, 9, 7);
    g.set_val(7, 1, 3);
    g.set_val(7, 2, 4);
    g.set_val(7, 3, 5);
    g.set_val(7, 4, 6);
    g.set_val(7, 5, 7);
    g.set_val(7, 6, 8);
    g.set_val(7, 7, 9);
    g.set_val(7, 8, 1);
    g.set_val(7, 9, 2);
    g.set_val(8, 1, 6);
    g.set_val(8, 2, 7);
    g.set_val(8, 3, 8);
    g.set_val(8, 4, 9);
    g.set_val(8, 5, 1);
    g.set_val(8, 6, 2);
    g.set_val(8, 7, 3);
    g.set_val(8, 8, 4);
    g.set_val(8, 9, 5);
    g.set_val(9, 1, 9);
    g.set_val(9, 2, 1);
    g.set_val(9, 3, 2);
    g.set_val(9, 4, 3);
    g.set_val(9, 5, 4);
    g.set_val(9, 6, 5);
    g.set_val(9, 7, 6);
    g.set_val(9, 8, 7);
    g.set_val(9, 9, 8);
    g.display();
}

#[test]
fn check_test() {
    let mut g = Grid::new();
    assert_eq!(false, g.check_puzzle());
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
    g.set_val(5, 4, 8);
    g.set_val(5, 5, 9);
    g.set_val(5, 6, 1);
    g.set_val(5, 7, 2);
    g.set_val(5, 8, 3);
    g.set_val(5, 9, 4);
    g.set_val(6, 1, 8);
    g.set_val(6, 2, 9);
    g.set_val(6, 3, 1);
    g.set_val(6, 4, 2);
    g.set_val(6, 5, 3);
    g.set_val(6, 6, 4);
    g.set_val(6, 7, 5);
    g.set_val(6, 8, 6);
    g.set_val(6, 9, 7);
    g.set_val(7, 1, 3);
    g.set_val(7, 2, 4);
    g.set_val(7, 3, 5);
    g.set_val(7, 4, 6);
    g.set_val(7, 5, 7);
    g.set_val(7, 6, 8);
    g.set_val(7, 7, 9);
    g.set_val(7, 8, 1);
    g.set_val(7, 9, 2);
    g.set_val(8, 1, 6);
    g.set_val(8, 2, 7);
    g.set_val(8, 3, 8);
    g.set_val(8, 4, 9);
    g.set_val(8, 5, 1);
    g.set_val(8, 6, 2);
    g.set_val(8, 7, 3);
    g.set_val(8, 8, 4);
    g.set_val(8, 9, 5);
    g.set_val(9, 1, 9);
    g.set_val(9, 2, 1);
    g.set_val(9, 3, 2);
    g.set_val(9, 4, 3);
    g.set_val(9, 5, 4);
    g.set_val(9, 6, 5);
    g.set_val(9, 7, 6);
    g.set_val(9, 8, 7);
    g.set_val(9, 9, 8);
    assert_eq!(true, g.check_puzzle());
}
