use super::super::objects::cardinal::*;
use super::super::objects::cell::*;
use super::super::objects::coordconverter::*;
use super::super::objects::grid::*;

pub struct ResolverLvl2 {
    cc: CoordConverter,
}

impl ResolverLvl2 {
    pub fn new(side: u16) -> ResolverLvl2 {
        ResolverLvl2 {
            cc: CoordConverter::new(side),
        }
    }

    /*
    for a square and a value, if all lines but one and all columns but one are solved then
    the cell in the remainig line and column is solved with this value

    return true if found at least a new value for a cell
    */
    pub fn resolve(&self, g: &mut Grid) -> bool {
        if g.resolved() {
            return false;
        }
        g.clear_trace();
        let mut resolve_some = false;
        //iter on squares
        let squ = Cardinal::C;
        let max = g.get_metrics().get_max();
        for sq in squ.get_all() {
            //iter on values
            for value in 1..=max {
                if self.resolve_square_val(g, sq, value) {
                    resolve_some = true
                }
            }
        }
        resolve_some
    }
    /**
     * check a value in a square
     * return true if a new cell is solved
     */
    fn resolve_square_val(&self, g: &mut Grid, squ: Cardinal, value: u16) -> bool {
        //check if the value is already in the square
        if g.check_value_in_square(squ, value) {
            return false;
        }

        //check if all but one line solved
        let mut unsolved_line = 255;
        for l in squ.get_lines() {
            //if unsolved in this line
            if !g.check_value_in_line(l, value) {
                //first unsolved ?
                if unsolved_line == 255 {
                    unsolved_line = l;
                } else {
                    //if two lines unsolved then let go
                    return false;
                }
            }
        }
        if unsolved_line == 255 {
            //if tree line solved then let go
            return false;
        }
        //now check the columns
        //check if all but one column solved
        let mut unsolved_column = 255;
        for c in squ.get_columns() {
            //if unsolved in this column
            if !g.check_value_in_column(c, value) {
                //first unsolved ?
                if unsolved_column == 255 {
                    unsolved_column = c;
                } else {
                    //if two columns unsolved then let go
                    return false;
                }
            }
        }
        if unsolved_column == 255 {
            //if tree columns solved then let go
            return false;
        }
        //check if cell is already solved
        let pos: usize = self.cc.coord_to_pos(unsolved_line, unsolved_column);
        let cell: &mut Cell = g.get_cell(pos);
        if cell.is_resolved() {
            return false;
        }
        //at this point only one line and one column unsolved => it is now
        g.set_val(unsolved_line, unsolved_column, value, CellType::Found);
        let trc = format!(" {}/{}={}", unsolved_line, unsolved_column, value);
        g.add_trace(trc);
        true
    }
}
