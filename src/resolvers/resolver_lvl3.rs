use super::super::objects::accessor::*;
use super::super::objects::cardinal::*;
use super::super::objects::cell::*;
use super::super::objects::grid::*;

use std::convert::TryInto;

pub struct ResolverLvl3 {
    acc: Accessor,
}

impl ResolverLvl3 {
    pub fn new(side: u8) -> ResolverLvl3 {
        ResolverLvl3 {
            acc: Accessor::new(side),
        }
    }

    /*
    If a value is not in the possible of a line less a cell then the cell has this values
    idem for column
    and square
    return true if found one or more
    */
    pub fn resolve(&self, g: &mut Grid) -> bool {
        if g.resolved() {
            return false;
        }
        let max = g.get_metrics().get_max();
        let nb_column = g.get_metrics().get_nb_column();
        let nb_line = g.get_metrics().get_nb_line();
        g.clear_trace();
        let mut solve_one_at_least = false;
        for v in 1..=max {
            let val: usize = v.try_into().unwrap();
            for line in 1..=nb_column {
                if self.resolve_line(g, line, val) {
                    solve_one_at_least = true;
                }
            }
            for column in 1..=nb_line {
                if self.resolve_column(g, column, val) {
                    solve_one_at_least = true;
                }
            }
            let c = Cardinal::C;
            for square in c.get_all() {
                if self.resolve_square(g, square, val) {
                    solve_one_at_least = true;
                }
            }
        }
        solve_one_at_least
    }

    fn resolve_line(&self, g: &mut Grid, line: u8, val: usize) -> bool {
        if g.check_value_in_line(line, val.try_into().unwrap()) {
            //if val already solved in the line
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_line(line) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = g.get_cell(pos);
            if cell.candidate(val) {
                if unsolve != 255 {
                    //second possible? -> 2 possibles -> not a solution
                    return false;
                }
                unsolve = p;
            }
        }
        if unsolve != 255 {
            //found
            let pos: usize = unsolve.try_into().unwrap();
            let v: u8 = val.try_into().unwrap();
            let coord = self.acc.coordconverter.pos_to_coord(pos);
            g.set_val(coord.0, coord.1, v, CellType::Found);
            let trc = format!(" l:{}/{}={}", coord.0, coord.1, val);
            g.add_trace(trc);
            return true;
        }
        false
    }

    fn resolve_column(&self, g: &mut Grid, column: u8, val: usize) -> bool {
        if g.check_value_in_column(column, val.try_into().unwrap()) {
            //if val already solved in the column
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_column(column) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = g.get_cell(pos);
            if cell.candidate(val) {
                if unsolve != 255 {
                    //second possible? -> 2 possibles -> not a solution
                    return false;
                }
                unsolve = p;
            }
        }
        if unsolve != 255 {
            //found
            let pos: usize = unsolve.try_into().unwrap();
            let v: u8 = val.try_into().unwrap();
            let coord = self.acc.coordconverter.pos_to_coord(pos);
            g.set_val(coord.0, coord.1, v, CellType::Found);
            let trc = format!(" c:{}/{}={}", coord.0, coord.1, val);
            g.add_trace(trc);
            return true;
        }
        false
    }

    fn resolve_square(&self, g: &mut Grid, square: Cardinal, val: usize) -> bool {
        if g.check_value_in_square(square, val.try_into().unwrap()) {
            //if val already solved in the square
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_square(square) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = g.get_cell(pos);
            if cell.candidate(val) {
                if unsolve != 255 {
                    //second possible? -> 2 possibles -> not a solution
                    return false;
                }
                unsolve = p;
            }
        }
        if unsolve != 255 {
            //found
            let pos: usize = unsolve.try_into().unwrap();
            let v: u8 = val.try_into().unwrap();
            let coord = self.acc.coordconverter.pos_to_coord(pos);
            g.set_val(coord.0, coord.1, v, CellType::Found);
            let trc = format!(" s:{}/{}={}", coord.0, coord.1, val);
            g.add_trace(trc);
            return true;
        }
        false
    }
}
