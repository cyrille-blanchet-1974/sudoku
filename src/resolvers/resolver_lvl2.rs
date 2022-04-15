use super::super::objects::accessor::*;
use super::super::objects::cardinal::*;
use super::super::objects::cell::*;
use super::super::objects::constant::*;
use super::super::objects::grid::*;

pub struct ResolverLvl2 {
    trace: String,
}

impl ResolverLvl2 {
    pub fn new() -> ResolverLvl2 {
        ResolverLvl2 {
            trace: String::new(),
        }
    }

    /*
        get a string containg what was found
    */
    pub fn get_trace(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.trace);
        output
    }

    /*
    for a square and a value, if all lines but one and all columns but one are solved then
    the cell in the remainig line and column is solved with this value

    return true if found at least a new value for a cell
    */
    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        if g.resolved() {
            return false;
        }
        self.trace = "".to_string();
        let mut resolve_some = false;
        //iter on squares
        let squ = Cardinal::C;
        for sq in squ.get_all() {
            //iter on values
            for value in 1..=MAX {
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
    fn resolve_square_val(&mut self, g: &mut Grid, squ: Cardinal, value: u8) -> bool {
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
        let pos: usize = coord_to_pos(unsolved_line, unsolved_column);
        let cell: &mut Cell = g.get_cell(pos);
        if cell.is_resolved() {
            return false;
        }
        //at this point only one line and one column unsolved => it is now
        g.set_val(unsolved_line, unsolved_column, value, CellType::Found);
        let trc = format!(" {}/{}={}", unsolved_line, unsolved_column, value);
        self.trace.push_str(&trc);
        true
    }
}
